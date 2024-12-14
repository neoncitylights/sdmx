use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type LocalizedText = HashMap<String, String>;
pub type Links = Vec<String>;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Action {
	Append,
	Replace,
	Delete,
	#[default]
	Information,
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

pub type Sender = Exchanger;
pub type Receiver = Exchanger;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Exchanger {
	pub id: String,
	pub name: String,
	pub names: LocalizedText,
	pub contacts: Vec<Contact>,
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
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	pub sender: Sender,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub receiver: Option<Vec<Receiver>>,
	pub links: Links,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum NumberOrString {
	Number(isize),
	String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum SdmxValue {
	String(String),
	Number(isize),
	Boolean(bool),
	LocalizedText(LocalizedText),
	Array(Box<Vec<SdmxValue>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct SdmxObject(pub HashMap<String, SdmxValue>);
