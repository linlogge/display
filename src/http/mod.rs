use reqwest::Client;
use serde::{Serialize};
use serde_json::{Map, Value};
use sha1::{Sha1, Digest};
use urlencoding::{decode};

fn hash_json(json: String) -> String {
    let json = decode(&json).unwrap();
    let json_user = "Linionik_HTML5";
    let json_password = "#_Linionik_6_!HTML5!#";
    let json_enc_str = format!("{}{}{}{}", json_user, json, json_password, "_Linionik_2012");

    let mut hasher = Sha1::new();
    hasher.update(json_enc_str);
    let json_sha1 = hex::encode(hasher.finalize());
    let json_ext_comm = format!("{}_[MC_JSON]_{}_[MC_JSON]_{}", json_user, json_sha1, json);
    json_ext_comm
}

pub async fn get<T: Serialize>(json: T) -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
    let host = "192.168.20.160";
    let protocol = "http";
    let path = "/json.php";

    let query_parameters = {
        let encoded_str = hash_json(serde_json::to_string(&json)?);
        vec![("Json", encoded_str)]
    };

    let mut url = url::Url::parse(&format!("{}://{}", protocol, host))?;
    url.set_path(path);
    url.query_pairs_mut().extend_pairs(query_parameters);

    let client = Client::new();
    let res = client.get(url).send().await?;

    let body = res.text().await?;

    let res: Map<String, Value> = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&body)?;

    Ok(res)
}

/* Future<Map<String, dynamic>> get(Map<String, dynamic> json) async {
  final encodedStr = hashJson(Uri.encodeComponent(jsonEncode(json)));
  final queryParameters = {
    'Json': encodedStr,
  };
  final url = Uri.http(_host, "/json.php", queryParameters);

  try {
    final response = await http.get(url);

    if (response.statusCode == 200) {
      final decodedResponse = jsonDecode(utf8.decode(response.bodyBytes));
      return decodedResponse;
    } else {
      print(response.body);
      throw Exception('Failed to load data');
    }
  } catch (e) {
    throw Exception(e);
  }
} */

/* static String hashJson(String str) {
    var decodedJson = Uri.decodeComponent(str);
    const jsonUser = "Linionik_HTML5";
    const jsonPassword = "#_Linionik_6_!HTML5!#";
    final jsonEncStr = '${jsonUser + decodedJson}${jsonPassword}_Linionik_2012';
    final jsonSha1 = sha1.convert(utf8.encode(jsonEncStr)).toString();
    final jsonExtComm = "${jsonUser}_[MC_JSON]_${jsonSha1}_[MC_JSON]_$str";
    return jsonExtComm;
  } */