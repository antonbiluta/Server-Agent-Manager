use maxminddb::geoip2;
use public_ip_addr::get_public_ip;
use std::error::Error;

pub async fn get_location() -> Result<(String, String, String), Box<dyn Error>> {
    let city_reader = maxminddb::Reader::open_readfile("GeoLite2-City.mmdb")?;
    let country_reader = maxminddb::Reader::open_readfile("GeoLite2-Country.mmdb")?;
    let ip = get_public_ip().await.unwrap();

    let city: geoip2::City = city_reader.lookup(ip.parse()?)?;
    let country: geoip2::Country = country_reader.lookup(ip.parse()?)?;

    let country_name = country.country
        .and_then(|c| c.names)
        .and_then(|n| n.get("en").cloned())
        .unwrap_or_else(|| "Unknown Country");

    let city_name = city.city
        .and_then(|c| c.names)
        .and_then(|n| n.get("en").cloned())
        .unwrap_or_else(|| "Unknown City");

    Ok((ip.to_string(), country_name.to_string(), city_name.to_string()))
}