use serde::Serialize;

#[derive(Debug, PartialOrd, PartialEq, Clone, Serialize)]
pub enum SeverityLevel{
    WARNING,
    INFORMATIVE,
    OK,
    ERROR
} //precisa remover o severity leval após