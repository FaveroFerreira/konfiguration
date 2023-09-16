use crate::error::KonfigurationError;
use crate::map::Map;

pub type TomlValue = toml::Value;

pub type ConfigurationManifest = Map<String, ConfigurationEntry>;

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigurationEntry {
    /// Represents a TOML string
    String(String),

    /// Represents a TOML integer
    Integer(i64),

    /// Represents a TOML float
    Float(f64),

    /// Represents a TOML boolean
    Boolean(bool),

    /// Represents a TOML array
    Array(Vec<ConfigurationEntry>),

    /// See `DetailedConfigurationEntry` for more information.
    Detailed(Box<DetailedConfigurationEntry>),

    /// Represents a TOML table that is not in the above format.
    Table(Map<String, ConfigurationEntry>),
}

/// Represents a TOML table in the following format:
/// ```toml
/// my-key = { env = "value" }
/// ```
/// or
/// ```toml
/// my-key = { env = "value", default = "default-value" }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DetailedConfigurationEntry {
    pub env: String,
    pub default: Box<Option<ConfigurationEntry>>,
}

impl ConfigurationEntry {
    /// Index into a TOML array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    pub fn get<I: Index>(&self, index: I) -> Option<&ConfigurationEntry> {
        index.index(self)
    }

    /// Mutably index into a TOML array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut ConfigurationEntry> {
        index.index_mut(self)
    }

    /// Extracts the integer value if it is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match *self {
            ConfigurationEntry::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Tests whether this value is an integer.
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    /// Extracts the float value if it is a float.
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            ConfigurationEntry::Float(f) => Some(f),
            _ => None,
        }
    }

    /// Tests whether this value is a float.
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    /// Extracts the boolean value if it is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            ConfigurationEntry::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Tests whether this value is a boolean.
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    /// Extracts the string of this value if it is a string.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            ConfigurationEntry::String(ref s) => Some(&**s),
            _ => None,
        }
    }

    /// Tests if this value is a string.
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    /// Extracts the array value if it is an array.
    pub fn as_array(&self) -> Option<&Vec<ConfigurationEntry>> {
        match *self {
            ConfigurationEntry::Array(ref s) => Some(s),
            _ => None,
        }
    }

    /// Extracts the array value if it is an array.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<ConfigurationEntry>> {
        match *self {
            ConfigurationEntry::Array(ref mut s) => Some(s),
            _ => None,
        }
    }

    /// Tests whether this value is an array.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// Extracts the table value if it is a table.
    pub fn as_table(&self) -> Option<&Map<String, ConfigurationEntry>> {
        match *self {
            ConfigurationEntry::Table(ref s) => Some(s),
            _ => None,
        }
    }

    /// Extracts the table value if it is a table.
    pub fn as_table_mut(&mut self) -> Option<&mut Map<String, ConfigurationEntry>> {
        match *self {
            ConfigurationEntry::Table(ref mut s) => Some(s),
            _ => None,
        }
    }

    /// Tests whether this value is a table.
    pub fn is_table(&self) -> bool {
        self.as_table().is_some()
    }

    /// Returns a human-readable representation of the type of this value.
    pub fn type_str(&self) -> &'static str {
        match *self {
            ConfigurationEntry::String(..) => "string",
            ConfigurationEntry::Integer(..) => "integer",
            ConfigurationEntry::Float(..) => "float",
            ConfigurationEntry::Boolean(..) => "boolean",
            ConfigurationEntry::Array(..) => "array",
            ConfigurationEntry::Table(..) => "table",
            ConfigurationEntry::Detailed(..) => "detailed",
        }
    }
}

impl TryFrom<TomlValue> for ConfigurationEntry {
    type Error = KonfigurationError;

    fn try_from(value: TomlValue) -> Result<Self, Self::Error> {
        match value {
            TomlValue::String(s) => Ok(ConfigurationEntry::String(s)),
            TomlValue::Integer(i) => Ok(ConfigurationEntry::Integer(i)),
            TomlValue::Float(f) => Ok(ConfigurationEntry::Float(f)),
            TomlValue::Boolean(b) => Ok(ConfigurationEntry::Boolean(b)),
            TomlValue::Datetime(d) => Err(KonfigurationError::Entry(format!(
                "Datetime is not supported: {}",
                d
            ))),
            TomlValue::Array(a) => {
                let array = a
                    .into_iter()
                    .map(|value| ConfigurationEntry::try_from(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ConfigurationEntry::Array(array))
            }
            TomlValue::Table(t) => {
                let table = t
                    .into_iter()
                    .map(|(key, value)| {
                        let entry = ConfigurationEntry::try_from(value).unwrap();
                        (key, entry)
                    })
                    .collect::<Map<_, _>>();

                Ok(ConfigurationEntry::Table(table))
            }
        }
    }
}

pub trait Index {
    #[doc(hidden)]
    fn index<'a>(&self, val: &'a ConfigurationEntry) -> Option<&'a ConfigurationEntry>;
    #[doc(hidden)]
    fn index_mut<'a>(&self, val: &'a mut ConfigurationEntry) -> Option<&'a mut ConfigurationEntry>;
}

impl Index for usize {
    fn index<'a>(&self, val: &'a ConfigurationEntry) -> Option<&'a ConfigurationEntry> {
        match *val {
            ConfigurationEntry::Array(ref a) => a.get(*self),
            _ => None,
        }
    }

    fn index_mut<'a>(&self, val: &'a mut ConfigurationEntry) -> Option<&'a mut ConfigurationEntry> {
        match *val {
            ConfigurationEntry::Array(ref mut a) => a.get_mut(*self),
            _ => None,
        }
    }
}

impl Index for str {
    fn index<'a>(&self, val: &'a ConfigurationEntry) -> Option<&'a ConfigurationEntry> {
        match *val {
            ConfigurationEntry::Table(ref a) => a.get(self),
            _ => None,
        }
    }

    fn index_mut<'a>(&self, val: &'a mut ConfigurationEntry) -> Option<&'a mut ConfigurationEntry> {
        match *val {
            ConfigurationEntry::Table(ref mut a) => a.get_mut(self),
            _ => None,
        }
    }
}

impl Index for String {
    fn index<'a>(&self, val: &'a ConfigurationEntry) -> Option<&'a ConfigurationEntry> {
        self[..].index(val)
    }

    fn index_mut<'a>(&self, val: &'a mut ConfigurationEntry) -> Option<&'a mut ConfigurationEntry> {
        self[..].index_mut(val)
    }
}

impl<'s, T: ?Sized> Index for &'s T
where
    T: Index,
{
    fn index<'a>(&self, val: &'a ConfigurationEntry) -> Option<&'a ConfigurationEntry> {
        (**self).index(val)
    }

    fn index_mut<'a>(&self, val: &'a mut ConfigurationEntry) -> Option<&'a mut ConfigurationEntry> {
        (**self).index_mut(val)
    }
}
