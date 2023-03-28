// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use chrono::prelude::*;
use minidom::Element;
use std::collections::HashMap;

pub type ContainerDatesMap = HashMap<String, (DateTime<Local>, DateTime<Local>)>;

/// Ask the FTI servers when the different containers at a recycling site was
/// emptied and when it will be done next time.
///
/// # Return
///
/// A hashmap where the key-value pair is the recycling container name and a
/// DateTime tuple. The first DateTime signifies when the container was
/// emptied, the second one tells when it's scheduled to be emptied.
pub fn fetch_recycling_station_status(id: u32) -> Result<ContainerDatesMap> {
    const URL: &'static str = "https://ftiws.ftiab.se/fti_ws/fti_ws.asmx?op=GetAVSStatistik";
    let envelope = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
           <soap12:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                            xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                            xmlns:soap12="http://www.w3.org/2003/05/soap-envelope">
             <soap12:Body>
               <GetAVSStatistik xmlns="http://tempuri.org/">
                 <lAvsId>{id}</lAvsId>
               </GetAVSStatistik>
             </soap12:Body>
           </soap12:Envelope>"#
    );
    let text = ureq::post(URL)
        .set("SOAPAction", "http://tempuri.org/GetAVSStatistik")
        .set("Content-Type", "text/xml;charset=UTF-8")
        .send_string(&envelope)
        .context("Could not send request to FTI")?
        .into_string()
        .context("FTI response couldn't be made into a string")?;
    extract_scheduled_events(text)
}

fn extract_scheduled_events(info: String) -> Result<ContainerDatesMap> {
    const NAMESPACE: &'static str = "http://tempuri.org/";
    const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    // Figure out where the list of recycling bins is
    let root: Element = info.parse().context("FTI response isn't valid XML")?;
    let body = root
        .get_child("Body", "http://www.w3.org/2003/05/soap-envelope")
        .context("No body found")?;
    let response = body
        .get_child("GetAVSStatistikResponse", NAMESPACE)
        .context("No response found")?;
    let statistics_result = response
        .get_child("GetAVSStatistikResult", NAMESPACE)
        .context("No statistics result found")?;

    // Store the recycled material, when last emptied and the scheduled time
    let mut map = HashMap::new();
    for stats in statistics_result.children() {
        // Site cleaning is listed as a material, but we don't care
        let material = stats
            .get_child("Material", NAMESPACE)
            .context("Missing material")?
            .text();
        if material == "Städning" {
            continue;
        }

        // Get last emptied and scheduled
        let emptied = stats
            .get_child("SenAktivitet", NAMESPACE)
            .context("Missing emptied information")?
            .text();
        let scheduled = stats
            .get_child("NastaAktivitet", NAMESPACE)
            .context("Missing scheduled activity")?
            .text();
        let emptied = Local
            .datetime_from_str(&emptied, DATE_FORMAT)
            .context("Bad format for emptied time")?;
        let scheduled = Local
            .datetime_from_str(&scheduled, DATE_FORMAT)
            .context("Bad format for scheduled time")?;

        // Store if the schedule specifies a year in the current milennium
        if scheduled.year() > 2000 {
            map.insert(material, (emptied, scheduled));
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    // A somewhat shortened XML response from the FTI server. This includes a
    // cleaning event ("Städning"), two scheduled emptying events
    // ("Metallförpackningar" and "Pappersförpackningar") as well as a 'at the
    // regular interval' event ("Tidningar").
    const XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope"
                   xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                   xmlns:xsd="http://www.w3.org/2001/XMLSchema">
      <soap:Body>
        <GetAVSStatistikResponse xmlns="http://tempuri.org/">
          <GetAVSStatistikResult>
            <AVSStatistik>
              <AVSid>12345</AVSid>
              <Material>Städning</Material>
              <Materialid>0</Materialid>
              <SenAktivitet>2023-03-27T13:18:11</SenAktivitet>
              <NastaAktivitet>2023-03-31T00:00:00</NastaAktivitet>
            </AVSStatistik>
            <AVSStatistik>
              <AVSid>12345</AVSid>
              <Material>Metallförpackningar</Material>
              <Materialid>3</Materialid>
              <SenAktivitet>2023-03-07T14:21:38</SenAktivitet>
              <NastaAktivitet>2023-04-04T00:00:00</NastaAktivitet>
            </AVSStatistik>
            <AVSStatistik>
              <AVSid>12345</AVSid>
              <Material>Pappersförpackningar</Material>
              <Materialid>5</Materialid>
              <SenAktivitet>2023-03-28T09:15:26</SenAktivitet>
              <NastaAktivitet>2023-03-29T00:00:00</NastaAktivitet>
            </AVSStatistik>
            <AVSStatistik>
              <AVSid>12345</AVSid>
              <Material>Tidningar</Material>
              <Materialid>13</Materialid>
              <SenAktivitet>2023-03-28T07:35:12</SenAktivitet>
              <NastaAktivitet>1900-01-01T00:00:00</NastaAktivitet>
            </AVSStatistik>
          </GetAVSStatistikResult>
        </GetAVSStatistikResponse>
      </soap:Body>
    </soap:Envelope>"#;

    #[test]
    fn test_process_recycling_infostr() -> Result<()> {
        let info = extract_scheduled_events(XML.to_string())?;
        assert_eq!(info.len(), 2); // Only two scheduled events
        Ok(())
    }
}
