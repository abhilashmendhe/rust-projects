use serde::{de::value::Error, Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct MoreDogs {
    dogs: Vec<Dog>
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all="camelCase")]
struct Dog {
    name: String, 
    year_born: i32,
    // owner: DogOwner
}
#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all="camelCase")]
struct DogOwner {
    first_name: String,
    last_name: String
}
// fn main() {
//     // let owner = DogOwner {
//     //     first_name: "Abhi".to_string(),
//     //     last_name: "mendhe".to_string()
//     // };
//     // let dog1 = Dog{
//     //     name: "piku".to_string(),
//     //     year_born: 2019,
//     //     owner 
//     // };
//     // let dog_ser = to_string_pretty(&dog1);
//     // if dog_ser.is_ok() {
//     //     println!("{}",dog_ser.ok().unwrap());
//     // } else {
//     //     println!("{:#?}",dog_ser.err());
//     // }

//     // deserialize();
//     let _ = multiple_deserialize();

// }
#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    data: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Foos {
    foos: Vec<Foo>,
}
fn main() -> Result<(), serde_json::Error> {
    let data = r#"[
        {
            "data": "value1"
        },
        {
            "data": "value2"
        },
        {
            "data": "value3"
        }
    ]"#;

    // let datas: Vec<Foo> = serde_json::from_str(data)?;

    // for data in datas.iter() {
    //     println!("{:#?}", data);
    // }
    let foos: Foos = serde_json::from_str(data)?;
    println!("{:?}",foos);
    let ans = multiple_deserialize()?;
    Ok(())
}
fn multiple_deserialize()  -> Result<(), serde_json::Error> {
    let json_string = r#"[
        {
            "name": "piku",
            "year_born": 2019
        },
        {
            "name": "tedu",
            "year_born": 2020
        }
      ]"#;
    //   let datas: Vec<Dog> = serde_json::from_str(&json_string).expect("vec of objects");
    let datas:MoreDogs = from_str(json_string)?;
    println!("{:?}",datas);
    Ok(())

    // if let Ok(dogs_derser: Vec<MoreDogs>) = from_str::<MoreDogs>(&json_string) {
    //     println!("{:?}",dogs_deser);
    // } else {
    //     println!("Erro deserializing!");
    // }
    
}

fn deserialize() {
    let json_string = r#"{
  "name": "piku",
  "year_born": 2019,
  "owner": {
    "first_name": "Abhi",
    "last_name": "mendhe"
  }
}"#;

    let dog_deser = from_str::<Dog>(&json_string);
    if dog_deser.is_ok() {
        println!("{:#?}",dog_deser.ok().unwrap());
    } else {
        println!("{:#?}",dog_deser.err());
    }
}


