use ic_cdk::{update, query};
use ic_cdk::api::management_canister::http_request::{
  http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
  TransformContext,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ResponseJson {
  message: String,
  // no timestamp for ic consensus
}

#[update]
async fn greet(name: String) -> String {
  let host = "jsontest.deno.dev";
  let url = format!("https://{}/?name={}", host, urlencoding::encode(&name));

  let request_headers = vec![
    HttpHeader {
      name: "Host".to_string(),
      value: format!("{host}:443"),
    },
  ];

  // Used by CanisterHttpRequestArgument.transform
  let context = ResponseJson {
    message: "test".to_string(),
  };

  //note "CanisterHttpRequestArgument" and "HttpMethod" are declared in line 4
  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    method: HttpMethod::GET,
    body: None,
    max_response_bytes: Some(1000),
    transform: Some(TransformContext::from_name("transform".to_string(), serde_json::to_vec(&context).unwrap())),
    headers: request_headers,
  };
  
  let cycle = 4_000_000u128; // FIXME 
  match http_request(request, cycle).await {  
    Ok((response,)) => {
      let obj: ResponseJson = serde_json::from_slice(&response.body).unwrap();
      obj.message
    }
    Err((_, m)) => {  
      m
    }
  }
}

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
  let headers = vec![
    HttpHeader {
        name: "Content-Security-Policy".to_string(),
        value: "default-src 'self'".to_string(),
    },
    HttpHeader {
        name: "Referrer-Policy".to_string(),
        value: "strict-origin".to_string(),
    },
    HttpHeader {
        name: "Permissions-Policy".to_string(),
        value: "geolocation=(self)".to_string(),
    },
    HttpHeader {
        name: "Strict-Transport-Security".to_string(),
        value: "max-age=63072000".to_string(),
    },
    HttpHeader {
        name: "X-Frame-Options".to_string(),
        value: "DENY".to_string(),
    },
    HttpHeader {
        name: "X-Content-Type-Options".to_string(),
        value: "nosniff".to_string(),
    },
  ];

  let mut res = HttpResponse {
    status: raw.response.status.clone(),
    body: raw.response.body.clone(),
    headers,
    ..Default::default()
  };

  if res.status == 200 as u128 {
      let obj: ResponseJson = serde_json::from_slice(&raw.response.body).unwrap();
      res.body = serde_json::to_vec(&obj).unwrap();
  } else {
      ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
  }
  res
}