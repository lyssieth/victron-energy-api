use serde::Deserialize;
use serde_json::Value;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Deserialize)]
/// Represents all information an installation has.
pub struct Installation {
    #[serde(rename = "idSite")]
    pub site_id: i32,
    #[serde(rename = "accessLevel")]
    /// The access level of the requesting user
    pub access_level: i32,
    /// True if the requesting user owns this installation
    pub owner: bool,
    /// True if the requesting user is an admin for this installation
    pub is_admin: bool,
    pub name: String,
    pub identifier: String,
    #[serde(rename = "idUser")]
    /// Installation owner's id
    pub user_id: i32,
    #[serde(rename = "pvMax")]
    /// Maximum PV for this installation
    pub pv_max: i32,
    pub timezone: String,
    #[serde(rename = "phonenumber")]
    pub phone_number: Option<String>,
    pub notes: Option<String>,
    /// Installation geofence, in json format
    pub geofence: Option<String>,
    #[serde(rename = "geofenceEnabled")]
    pub geofence_enabled: bool,
    #[serde(rename = "realtimeUpdates")]
    pub realtime_updates: bool,
    #[serde(rename = "hasMains")]
    pub has_mains: i8,
    #[serde(rename = "hasGenerator")]
    pub has_generator: i8,
    #[serde(rename = "noDataAlarmTimeout")]
    /// How many seconds after no installation data is received an alarm should be triggered
    pub no_data_alarm_timeout: Option<i32>,
    #[serde(rename = "alarmMonitoring")]
    /// If alarms and warnings should be sent.
    ///
    /// - 0: Nothing is sent out
    /// - 1: Only alarms
    /// - 2: Alarms and warnings
    pub alarm_monitoring: i8,
    #[serde(rename = "invalidVRMAuthTokenUsedInLogRequest")]
    /// 1 if an invalid token was used for logging, else 0
    pub invalid_vrm_auth_token_used_in_log_request: i8,
    #[serde(rename = "syscreated")]
    /// Installation creation timestamp, UNIX timestamp
    pub sys_created: i64,
    pub shared: bool,
    pub device_icon: String,
    #[serde(default)]
    pub alarm: Option<bool>,
    #[serde(default)]
    /// Timestamp of the most recently received data, UNIX timestamp
    pub last_timestamp: Option<i64>,
    #[serde(default)]
    /// The current time of the installation in 24h format (hh:mm)
    pub current_time: Option<String>,
    #[serde(default)]
    /// How many seconds the installation is off from UTC
    pub timezone_offset: Option<i32>,
    #[serde(default)]
    pub demo_mode: Option<bool>,
    #[serde(default)]
    pub mqtt_webhost: Option<String>,
    #[serde(default)]
    pub mqtt_host: Option<String>,
    #[serde(default)]
    /// True if the D-Bus round trip time is higher than the threshold
    pub high_workload: Option<bool>,
    #[serde(default)]
    pub current_alarms: Option<Vec<String>>,
    #[serde(default)]
    pub num_alarms: Option<i32>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub images: Option<Vec<Image>>,
    #[serde(default)]
    /// Installation view permissions for the requesting user.
    pub view_permissions: Option<ViewPermissions>,
    #[serde(default)]
    /// A data attribute.
    ///
    /// # Note
    /// I'm not certain what exactly this is yet, but it's in the schema.
    #[serde(deserialize_with = "deserialize_data_attribute")]
    pub extended: Option<Vec<Extended>>,
}

fn deserialize_data_attribute<'de, D>(deserializer: D) -> Result<Option<Vec<Extended>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    value.as_array().map_or_else(
        || Ok(None),
        |array| {
            let extended = array
                .iter()
                .flat_map(|v| {
                    if v.get("code").is_some() {
                        serde_json::from_value(v.clone()).map(Extended::Summary)
                    } else {
                        serde_json::from_value(v.clone()).map(Extended::Data)
                    }
                })
                .collect();

            Ok(Some(extended))
        },
    )
}

#[derive(Debug, Clone)]
pub enum Extended {
    Data(Data),
    Summary(Summary),
}

impl Extended {
    #[must_use]
    /// Returns true if the extended information is data.
    pub const fn is_data(&self) -> bool {
        matches!(self, Self::Data(_))
    }

    #[must_use]
    /// Returns true if the extended information is a summary.
    pub const fn is_summary(&self) -> bool {
        matches!(self, Self::Summary(_))
    }

    #[must_use]
    /// Returns the data if it is data.
    pub const fn as_data(&self) -> Option<&Data> {
        match self {
            Self::Data(data) => Some(data),
            Self::Summary(_) => None,
        }
    }

    #[must_use]
    /// Returns the summary if it is a summary.
    pub const fn as_summary(&self) -> Option<&Summary> {
        match self {
            Self::Summary(summary) => Some(summary),
            Self::Data(_) => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Data {
    #[serde(rename = "idDataAttribute")]
    pub data_id: i32,
    pub code: String,
    pub description: String,
    #[serde(rename = "formatWithUnit")]
    pub format_with_unit: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "textValue")]
    pub text_value: String,
    pub instance: String,
    pub timestamp: String,
    #[serde(rename = "dbusServiceType")]
    pub dbus_service_type: String,
    #[serde(rename = "dbusPath")]
    pub dbus_path: String,
    #[serde(rename = "rawValue")]
    pub raw_value: String,
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "formattedValueWithUnit")]
    pub data_attribute_enum_values: Vec<DataAttributeEnumValue>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataAttributeEnumValue {
    #[serde(rename = "nameEnum")]
    pub name_enum: String,
    #[serde(rename = "valueEnum")]
    pub value_enum: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(rename = "idDataAttribute")]
    pub data_id: i32,
    pub code: String,
    pub description: String,
    #[serde(rename = "rawValue")]
    pub raw_value: Option<f64>,
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "textValue")]
    pub text_value: Option<String>,
    #[serde(rename = "formatWithUnit")]
    pub format_with_unit: String,
    #[serde(rename = "dataAttributes")]
    pub data_attributes: Vec<DataAttribute>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataAttribute {
    pub instance: i32,
    #[serde(rename = "dbusServiceType")]
    pub dbus_service_type: String,
    #[serde(rename = "dbusPath")]
    pub dbus_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    #[serde(rename = "idTag")]
    pub tag_id: i32,
    pub name: String,
    pub automatic: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    #[serde(rename = "idSiteImage")]
    pub image_id: i32,
    #[serde(rename = "imageName")]
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
/// Installation view permissions for the requesting user.
pub struct ViewPermissions {
    /// True if the requesting user can modify general settings.
    pub update_settings: bool,
    /// True if the requesting user can view general settings.
    pub settings: bool,
    /// True if the requesting user can view diagnostics.
    pub diagnostics: bool,
    /// True if the requesting user can modify site share settings.
    pub share: bool,
    /// True if the requesting user can view VNC.
    pub vnc: bool,
    /// True if the requesting user can view MQTT RPC.
    pub mqtt_rpc: bool,
    /// True if the requesting user can view VNC.
    pub vebus: bool,
    /// True if the installation has two way communication.
    pub twoway: bool,
    /// True if the requesting user can view the exact location.
    pub exact_location: bool,
    /// True if the installation has Node RED.
    pub nodered: bool,
    /// True if the installation has a Node RED Dashboard.
    pub nodered_dash: bool,
    /// True if the installation has SignalK.
    pub signalk: bool,
}
