
use serde::{Serialize,Serializer,Deserialize,Deserializer};
use serde::ser::{SerializeStruct};
use serde::de::{Visitor,MapAccess};
use serde::de;
use serde_json;

use std::{fmt,str};

#[derive(Debug,PartialEq)]
struct KubeConfig{
    port : u8,
    healthz_port : u8,
    max_pods: u8,
}

impl Serialize for KubeConfig{
    fn serialize<S>(&self,w: S) -> Result<S::Ok,S::Error>
        where S : Serializer
    {
        let mut s = w.serialize_struct("KubeConfig",3)?;
        s.serialize_field("port",&self.port)?;
        s.serialize_field("healthz_port",&self.healthz_port)?;
        s.serialize_field("max_pods",&self.max_pods)?;
        s.end()
    }
}

const FIELDS : &'static [&'static str] = &["port","healthz_port","max_pods"];

enum Field{
    Port,
    HealthzPort,
    MaxPods,
}

struct FieldVisitor;

impl<'de> Visitor<'de> for FieldVisitor{
    
    type Value = Field;

    fn expecting(&self,f : &mut fmt::Formatter) -> fmt::Result{
        f.write_str("'port' or 'healthz_port' or 'max_pods'")
    }

    fn visit_str<E>(self,value: &str) -> Result<Field,E> where E : de::Error
    {
        match value{
            "port" => Ok(Field::Port),
            "healthz_port" => Ok(Field::HealthzPort),
            "max_pods" => Ok(Field::MaxPods),
            _ => Err(de::Error::unknown_field(value,FIELDS)),
        }
    }
}


impl<'de> Deserialize<'de> for Field{
    fn deserialize<D>(w : D) -> Result<Field,D::Error> where D:Deserializer<'de>{
        w.deserialize_identifier(FieldVisitor) 
    }
}

//----------------

struct KubeConfigVisitor;

impl<'de> Visitor<'de> for KubeConfigVisitor{

    type Value = KubeConfig;

    fn expecting(&self,f : &mut fmt::Formatter) -> fmt::Result{
        f.write_str("struct KubConfig")
    }

    fn visit_map<V>(self,mut map : V) -> Result<KubeConfig,V::Error> 
        where V: MapAccess<'de>
    {
        // 自动推导类型为 Option<u8>
        // map.next_value() 自动调用 <u8 as Deserialize>::deserialize()
        let mut port = None;
        let mut hport = None;
        let mut max = None;

        // match 分支类型为 Field
        // 则 map.next_key() 自动调用 <Field as Deserialize>::deserialize()
        while let Some(key) = map.next_key()?{
            match key{
                Field::Port => {
                    if port.is_some(){
                        return Err(de::Error::duplicate_field("port"));
                    }else{
                        port = Some(map.next_value()?);
                    }
                },
                Field::HealthzPort => {
                    if hport.is_some(){
                        return Err(de::Error::duplicate_field("healthz_port"));
                    }else{
                        hport = Some(map.next_value()?);
                    }
                },
                Field::MaxPods => {
                    if max.is_some(){
                        return Err(de::Error::duplicate_field("max_pods"));
                    }else{
                        max = Some(map.next_value()?);
                    }
                },
            }
        }

        let port = port.ok_or_else(||de::Error::missing_field("port"))?;
        let hport = hport.ok_or_else(||de::Error::missing_field("healthz_port"))?;
        let max = max.ok_or_else(||de::Error::missing_field("max_pods"))?;

        Ok(KubeConfig{
            port: port,
            healthz_port: hport,
            max_pods: max,
        })
    }
}

impl<'de> Deserialize<'de> for KubeConfig{
    fn deserialize<D>(w : D) -> Result<KubeConfig,D::Error> where D:Deserializer<'de>{
        w.deserialize_struct("KubeConfig",FIELDS,KubeConfigVisitor)
    } 
}


//--------

#[test]
fn test_ser_de(){

    use serde_test::{Token,assert_de_tokens};
    
    let c = KubeConfig{
        port: 10,
        healthz_port: 11,
        max_pods: 12,
    };
    
    assert_de_tokens(&c,&[
        Token::Struct{name: "KubeConfig",len: 3},
        Token::Str("port"),Token::U8(10),
        Token::Str("healthz_port"),Token::U8(11),
        Token::Str("max_pods"),Token::U8(12),
        Token::StructEnd,
    ]);
}

fn main(){
    let c = KubeConfig{
        port: 10,
        healthz_port: 11,
        max_pods: 12,
    };

    let s = serde_json::to_string(&c).unwrap();
    println!("{:?}",s);
}

