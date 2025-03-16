/**
Error Handling
Rst membagi 2 error menjadi 2 jenis :
- recoverable : dapat dipulihkan
- unrecoverable : tidak dapat dipulihkan

Unrecoverable Error
Ini error yang tidak bisa dipulihkan dan akan membuat aplikasi mati
Untuk mentrigger error ini bisa menggunakna macro panic!

Recoverable Error
Ini error yang bisa dipulihkan atau bisa dihandle jika terjadi error, jadi aplikasi tidak langsung mati begitu saja
Karena rust tidak menggunakan pendekatan try catch, pada function yang bisa mengembalikan error bisa menggunakan return value Enum Result
Dalam Enum Result terdapat 2 tipe Ok(T) dan Error(E)

? Operator
Operator ini bisa mengembalikan result jika memang terjadi error

*/

fn connect_database(host: Option<String>) {
    match host {
        None => {
            panic!("No database host provided.");
        }
        Some(host) => {
            println!("Host : {}", host);
        }
    }
}

#[test]
fn test_unrecoverable_error() {
    connect_database(Some("postgres".to_string()));
    connect_database(None);
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err("No cache host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}


#[test]
fn test_recoverable_error() {
    // let cache = connect_cache(Some("localhost".to_string()));
    let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("Success connect : {}", host);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err("No email host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    //Berikut adalah cara lebih cepat menggunakan ?, jika terjadi error maka err akan langsung dilakukanr return
    connect_cache(host.clone())?;

    //Ini adalah cara manual
    let email_result = connect_email(host.clone());

    match email_result {
        Ok(_) => {println!("Haiyaaa Loo")}
        Err(err) => {
            return Err(err);
        }
    }

    Ok("Success connect application".to_string())
}

#[test]
fn connect_to_application() {
    let result = connect_application(None);
    match result {
        Ok(host) => {println!("Success connect to host : {}", host);}
        Err(err) => {println!("Error with message : {}", err);}
    }
}