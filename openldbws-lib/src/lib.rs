use roxmltree::Document;
use reqwest::Client;
use std::time::Duration;
use std::str;
use anyhow::{anyhow, Context, Result};

// Why are these macros and not consts?
// For some reason, format! does not support
// consts.

macro_rules! service_details {
    () => {"<soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:typ=\"http://thalesgroup.com/RTTI/2013-11-28/Token/types\" xmlns:ldb=\"http://thalesgroup.com/RTTI/2021-11-01/ldb/\"><soapenv:Header><typ:AccessToken><typ:TokenValue>{token}</typ:TokenValue></typ:AccessToken></soapenv:Header><soapenv:Body><ldb:GetServiceDetailsRequest><ldb:serviceID>{service}</ldb:serviceID></ldb:GetServiceDetailsRequest></soapenv:Body></soapenv:Envelope>"}
}

macro_rules! arrival_details {
    () => {"<soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:typ=\"http://thalesgroup.com/RTTI/2013-11-28/Token/types\" xmlns:ldb=\"http://thalesgroup.com/RTTI/2021-11-01/ldb/\"><soapenv:Header><typ:AccessToken><typ:TokenValue>{token}</typ:TokenValue></typ:AccessToken></soapenv:Header><soapenv:Body><ldb:GetArrivalBoardRequest><ldb:numRows>150</ldb:numRows><ldb:crs>{crs}</ldb:crs><ldb:filterCrs>{filter_crs}</ldb:filterCrs><ldb:filterType>{filter_type}</ldb:filterType><ldb:timeOffset>{time_offset}</ldb:timeOffset><ldb:timeWindow>{time_window}</ldb:timeWindow></ldb:GetArrivalBoardRequest></soapenv:Body></soapenv:Envelope>"}
}

pub enum Error {
    Request,
    Status,
    Parse
}


pub struct TrainService {
    sta: u16,
    eta: u16,
}

pub async fn get_service_details<'a>(client: Client, token: &str, service: &str) -> Result<Document<'a>> {
    let service_details_payload = format!(service_details!(), token=token, service=service);
    let res = client.post("https://lite.realtime.nationalrail.co.uk/OpenLDBWS/ldb12.asmx")
        .body(service_details_payload)
        .timeout(Duration::new(5, 0))
        .header("Content-Type", "text/xml")
        .header("Accept", "text/xml")
        .send()
        .await
        .context("failed to send request")?;

    let result = res.text().await.context("couldn't get response result")?;
    
    Document::parse(Box::leak(Box::new(result))).context("couldn't parse document")
}
