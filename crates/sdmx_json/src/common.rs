use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type LocalizedText = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Link {
	#[serde(flatten)]
	pub location: Location,
	pub rel: String,
	pub url: Option<String>,
	pub uri: Option<String>,
	pub title: Option<String>,
	pub titles: Option<LocalizedText>,
	#[serde(rename = "type")]
	pub type_: Option<String>,
	pub hreflang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Location {
	Href(String),
	Urn(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Action {
	Append,
	Replace,
	Delete,
	#[default]
	Information,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Annotation {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "type")]
	pub type_: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub texts: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub department: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub departments: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub telephones: Option<Vec<String>>,
	pub faxes: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uris: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub emails: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub x400s: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum DataType {
	String,
	Alpha,
	AlphaNumeric,
	Numeric,
	BigInteger,
	Integer,
	Long,
	Short,
	Decimal,
	Float,
	Double,
	Boolean,
	#[serde(rename = "URI")]
	Uri,
	Count,
	InclusiveValueRange,
	ExclusiveValueRange,
	Incremental,
	ObservationalTimePeriod,
	StandardTimePeriod,
	BasicTimePeriod,
	GregorianTimePeriod,
	GregorianYear,
	GregorianYearMonth,
	GregorianDay,
	ReportingTimePeriod,
	ReportingYear,
	ReportingSemester,
	ReportingTrimester,
	ReportingQuarter,
	ReportingMonth,
	ReportingWeek,
	ReportingDay,
	DateTime,
	TimeRange,
	Month,
	MonthDay,
	Day,
	Time,
	Duration,
	GeospatialInformation,
	#[serde(rename = "XHTML")]
	Xhtml,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Error {
	pub code: f64, // NOTE: Original schema specifies this as number instead of integer(?)
	pub title: String,
	pub titles: LocalizedText,
	pub detail: String,
	pub details: LocalizedText,
	pub links: Vec<Link>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Party {
	pub id: String,
	pub name: Option<String>,
	pub names: Option<LocalizedText>,
	pub contacts: Option<Vec<Contact>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

pub type Sender = Party;
pub type Receiver = Party;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetaSingleReceiver {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<String>,
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub test: Option<bool>,
	pub prepared: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_languages: Option<Vec<String>>,
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	pub sender: Sender,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub receiver: Option<Receiver>,
	pub links: Vec<Link>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetaManyReceivers {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<String>,
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub test: Option<bool>,
	pub prepared: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_languages: Option<Vec<String>>,
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	pub sender: Sender,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub receivers: Option<Vec<Receiver>>,
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum NumberOrString {
	Number(isize),
	String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SdmxValue {
	Null,
	String(String),
	Integer(isize),
	Number(f64),
	Boolean(bool),
	LocalizedText(LocalizedText),
	Array(Box<Vec<SdmxValue>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct SdmxObject(pub HashMap<String, SdmxValue>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SentinelValue {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<NumberOrString>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}
