use crate::structure::TimeDataType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;
use std::collections::HashMap;

/// A marker trait for denoting that an object is extendable,
/// where it can accept additional properties beyond those
/// defined in the SDMX-JSON schema.
pub trait Extendable {
	fn other(&self) -> Option<&HashMap<String, Value>>;
}

/// A marker trait for all top-level message types in the
/// SDMX-JSON standard.
pub trait SdmxMessage {
	type Data;
	fn meta(&self) -> Option<&Meta>;
	fn data(&self) -> Option<&Self::Data>;
	fn errors(&self) -> Option<&Vec<StatusMessage>>;
}

/// A map between languages and the associated content
/// in that language.
pub type LocalizedText = HashMap<String, String>;

/// A link to an external resource.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Link {
	#[serde(flatten)]
	pub location: Location,
	pub rel: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub uri: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub titles: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "type")]
	pub type_: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hreflang: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A reference to a digital location, which may either
/// be a hyperlink reference (href),
/// or a uniform resource name (URN)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Location {
	/// A hyperlink reference
	Href(String),
	/// A uniform resource name
	Urn(String),
}

impl Location {
	pub fn as_string(&self) -> &String {
		match self {
			Self::Href(s) | Self::Urn(s) => s,
		}
	}
}

/// An action which describes how or why the data is being transmitted
/// from the sender's side.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Action {
	Append,
	Replace,
	Delete,
	#[default]
	Information,
}

/// Extra information that may be attached to another object.
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

/// A collection of contact information for an individual.
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

/// The specific data format for representing something.
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

impl DataType {
	pub const fn is_reporting(&self) -> bool {
		matches!(
			self,
			Self::ReportingDay
				| Self::ReportingWeek
				| Self::ReportingMonth
				| Self::ReportingYear
				| Self::ReportingQuarter
				| Self::ReportingSemester
				| Self::ReportingTrimester
				| Self::ReportingTimePeriod
		)
	}

	pub const fn is_gregorian(&self) -> bool {
		matches!(
			self,
			Self::GregorianTimePeriod
				| Self::GregorianYear
				| Self::GregorianYearMonth
				| Self::GregorianDay
		)
	}
}

impl From<TimeDataType> for DataType {
	fn from(value: TimeDataType) -> Self {
		match value {
			TimeDataType::ObservationalTimePeriod => Self::ObservationalTimePeriod,
			TimeDataType::StandardTimePeriod => Self::StandardTimePeriod,
			TimeDataType::BasicTimePeriod => Self::BasicTimePeriod,
			TimeDataType::GregorianTimePeriod => Self::GregorianTimePeriod,
			TimeDataType::GregorianYear => Self::GregorianYear,
			TimeDataType::GregorianYearMonth => Self::GregorianYearMonth,
			TimeDataType::GregorianDay => Self::GregorianDay,
			TimeDataType::ReportingTimePeriod => Self::ReportingTimePeriod,
			TimeDataType::ReportingYear => Self::ReportingYear,
			TimeDataType::ReportingSemester => Self::ReportingSemester,
			TimeDataType::ReportingTrimester => Self::ReportingTrimester,
			TimeDataType::ReportingQuarter => Self::ReportingQuarter,
			TimeDataType::ReportingMonth => Self::ReportingMonth,
			TimeDataType::ReportingWeek => Self::ReportingWeek,
			TimeDataType::ReportingDay => Self::ReportingDay,
			TimeDataType::DateTime => Self::DateTime,
			TimeDataType::TimeRange => Self::TimeRange,
		}
	}
}

/// A message with an HTTP status code, presumably an error status code.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct StatusMessage {
	pub code: usize,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub titles: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub details: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// An individual responsible for transmitting/receiving a message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Party {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub contacts: Option<Vec<Contact>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// The party responsible for transmitting a message.
pub type Sender = Party;
/// The party responsible for receiving a message.
pub type Receiver = Party;

/// Non-standard information and basic technical information
/// associated with a message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct Meta {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<String>,
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub test: Option<bool>,
	pub prepared: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_languages: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	pub sender: Sender,
	#[serde_as(as = "OneOrMany<_, PreferOne>")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub receivers: Option<Vec<Receiver>>,
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A primitive for representing either a string or signed integer.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum NumberOrString {
	Number(isize),
	String(String),
}

/// A primitive for describing pure SDMX-JSON values.
///
/// This type may or may not be replaced with
/// [`serde_json::Value`] in the future.
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

/// An object of keys mapping to pure SDMX-JSON primitive values.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct SdmxObject(pub HashMap<String, SdmxValue>);

/// A reserved value within an associated data domain and
/// some semantic meaning.
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

impl_extendable!(
	Link,
	Annotation,
	Contact,
	StatusMessage,
	Party,
	Meta,
	SentinelValue,
);
