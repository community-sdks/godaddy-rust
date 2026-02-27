mod support;

use godaddy_rust::http::Response;
use godaddy_rust::{Client, Config};
use serde_json::json;

use support::MockTransport;

fn test_client(transport: std::sync::Arc<MockTransport>) -> Client {
    let mut config = Config::default();
    config.api_key = Some(String::from("key"));
    config.api_secret = Some(String::from("secret"));
    config.max_retries = 0;
    Client::with_transport(config, transport)
}

#[tokio::test]
async fn every_service_method_builds_a_request() {
    let transport = MockTransport::shared();
    let client = test_client(transport.clone());

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .abuse()
        .get_tickets(
            Some("sample".into()),
            Some(true.into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "abuse.get_tickets");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .abuse()
        .create_ticket(json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "abuse.create_ticket");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.abuse().get_ticket_info("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "abuse.get_ticket_info");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .abuse()
        .get_tickets_v2(
            Some("sample".into()),
            Some(true.into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "abuse.get_tickets_v2");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .abuse()
        .create_ticket_v2(json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "abuse.create_ticket_v2");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.abuse().get_ticket_info_v2("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "abuse.get_ticket_info_v2");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .aftermarket()
        .get_listings(
            "sample",
            Some(vec!["sample"].into()),
            Some(vec!["sample"].into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "aftermarket.get_listings");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .aftermarket()
        .delete_listings(vec!["sample"])
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "aftermarket.delete_listings");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .aftermarket()
        .add_expiry_listings(vec!["sample"])
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "aftermarket.add_expiry_listings"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .agreements()
        .get(
            vec!["sample"],
            Some("header-value".into()),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "agreements.get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .search_ans_name(
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.search_ans_name");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .register_agent(json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.register_agent");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .resolve_ans_name(json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.resolve_ans_name");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.ans().get_agent("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.get_agent");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.ans().validate_registration("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.validate_registration");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.ans().verify_dns_records("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.verify_dns_records");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .get_agent_identity_certificate_by_agent_id("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "ans.get_agent_identity_certificate_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .submit_agent_identity_csr_by_agent_id("sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "ans.submit_agent_identity_csr_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .get_agent_server_certificate_by_agent_id("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "ans.get_agent_server_certificate_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .submit_agent_server_csr_by_agent_id("sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "ans.submit_agent_server_csr_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .get_agent_csr_status_by_agent_id("sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "ans.get_agent_csr_status_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .get_agent_events(
            Some("header-value".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.get_agent_events");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .auctions()
        .place_bids("sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "auctions.place_bids");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_create(json!({"sample": true}), Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_create"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_validate(json!({"sample": true}), Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_validate"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_get("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "certificates.certificate_get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_action_retrieve("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_action_retrieve"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_resend_email("sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_resend_email"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_alternate_email_address("sample", vec!["sample"])
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_alternate_email_address"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_resend_email_address("sample", "sample", vec!["sample"])
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_resend_email_address"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_email_history("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_email_history"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_callback_delete("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_callback_delete"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_callback_get("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_callback_get"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_callback_replace("sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_callback_replace"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_cancel("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_cancel"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_download("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_download"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_reissue("sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_reissue"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_renew("sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "certificates.certificate_renew");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_revoke("sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_revoke"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_siteseal_get("sample", Some("sample".into()), Some("sample".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_siteseal_get"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_verifydomaincontrol("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_verifydomaincontrol"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_get_entitlement("sample", Some(true.into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_get_entitlement"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_create_v2(json!({"sample": true}), Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_create_v2"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_download_entitlement("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.certificate_download_entitlement"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .get_customer_certificates_by_customer_id("sample", Some(1_i64.into()), Some(1_i64.into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.get_customer_certificates_by_customer_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .get_certificate_detail_by_cert_identifier("sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.get_certificate_detail_by_cert_identifier"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .get_domain_information_by_certificate_id("sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.get_domain_information_by_certificate_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .get_domain_details_by_domain("sample", "sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.get_domain_details_by_domain"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .get_acme_external_account_binding("sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.get_acme_external_account_binding"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .retrieve_ssl_by_domain_reseller(
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("sample".into()),
            Some(vec!["sample"].into()),
            Some("sample".into()),
            Some("sample".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.retrieve_ssl_by_domain_reseller"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .retrieve_ssl_by_domain_subscription_reseller(
            "sample",
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("sample".into()),
            Some(vec!["sample"].into()),
            Some("sample".into()),
            Some("sample".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "certificates.retrieve_ssl_by_domain_subscription_reseller"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.countries().get_countries("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "countries.get_countries");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .countries()
        .get_country("sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "countries.get_country");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .list(
            Some("header-value".into()),
            Some(vec!["sample"].into()),
            Some(vec!["sample"].into()),
            Some(1_i64.into()),
            Some("sample".into()),
            Some(vec!["sample"].into()),
            Some("sample".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.list");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_agreement(
            vec!["sample"],
            true,
            Some("header-value".into()),
            Some(true.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.get_agreement");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .available("sample", Some("sample".into()), Some(true.into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.available");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .available_bulk(vec!["sample"], Some("sample".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.available_bulk");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .contacts_validate(
            json!({"sample": true}),
            Some("header-value".into()),
            Some("sample".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.contacts_validate");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .purchase(json!({"sample": true}), Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.purchase");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.domains().schema("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.schema");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .validate(json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.validate");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .suggest(
            Some("header-value".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(vec!["sample"].into()),
            Some(vec!["sample"].into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.suggest");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.domains().tlds().await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.tlds");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client.domains().cancel("sample").await.unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.cancel");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get("sample", Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .update(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PATCH");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.update");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .update_contacts(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PATCH");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.update_contacts");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .cancel_privacy("sample", Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.cancel_privacy");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .purchase_privacy(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.purchase_privacy");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .record_add(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PATCH");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.record_add");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .record_replace(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.record_replace");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .record_get(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.record_get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .record_replace_type_name(
            "sample",
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.record_replace_type_name"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .record_delete_type_name("sample", "sample", "sample", Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.record_delete_type_name"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .record_replace_type(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.record_replace_type");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .renew(
            "sample",
            Some("header-value".into()),
            Some(json!({"sample": true}).into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.renew");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .transfer_in(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.transfer_in");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .verify_email("sample", Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.verify_email");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_domain(
            "sample",
            "sample",
            Some("header-value".into()),
            Some(vec!["sample"].into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_domain"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .delete_v2_customers_customer_id_domains_domain_change_of_registrant(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.delete_v2_customers_customer_id_domains_domain_change_of_registrant"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_domain_change_of_registrant(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_domain_change_of_registrant"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .patch_v2_customers_customer_id_domains_domain_dnssec_records(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PATCH");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.patch_v2_customers_customer_id_domains_domain_dnssec_records"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .delete_v2_customers_customer_id_domains_domain_dnssec_records(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.delete_v2_customers_customer_id_domains_domain_dnssec_records"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .put_v2_customers_customer_id_domains_domain_name_servers(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.put_v2_customers_customer_id_domains_domain_name_servers"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_domain_privacy_forwarding(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_domain_privacy_forwarding"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .patch_v2_customers_customer_id_domains_domain_privacy_forwarding(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PATCH");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.patch_v2_customers_customer_id_domains_domain_privacy_forwarding"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_redeem(
            "sample",
            "sample",
            Some("header-value".into()),
            Some(json!({"sample": true}).into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_redeem"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_renew(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_renew"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_domain_transfer(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_domain_transfer"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_validate(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_validate"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_in_accept(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_in_accept"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_in_cancel(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_in_cancel"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_in_restart(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_in_restart"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_in_retry(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_in_retry"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_out(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_out"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_out_accept(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_out_accept"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_transfer_out_reject(
            "sample",
            "sample",
            Some("header-value".into()),
            Some("sample".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_transfer_out_reject"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .domains_forwards_delete("sample", "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.domains_forwards_delete"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .domains_forwards_get("sample", "sample", Some(true.into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.domains_forwards_get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .domains_forwards_put("sample", "sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.domains_forwards_put");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .domains_forwards_post("sample", "sample", json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.domains_forwards_post");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_domain_actions(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_domain_actions"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .delete_v2_customers_customer_id_domains_domain_actions_type(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.delete_v2_customers_customer_id_domains_domain_actions_type"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_domain_actions_type(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_domain_actions_type"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_notifications("sample", Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_notifications"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_notifications_opt_in(
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_notifications_opt_in"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .put_v2_customers_customer_id_domains_notifications_opt_in(
            "sample",
            vec!["sample"],
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.put_v2_customers_customer_id_domains_notifications_opt_in"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_notifications_schemas_type(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_notifications_schemas_type"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_notifications_notification_id_acknowledge(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_notifications_notification_id_acknowledge"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_register(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_register"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_customers_customer_id_domains_register_schema_tld(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_customers_customer_id_domains_register_schema_tld"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_register_validate(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_register_validate"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_domains_maintenances(
            Some("header-value".into()),
            Some(vec!["sample"].into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_domains_maintenances"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_domains_maintenances_maintenance_id("sample", Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_domains_maintenances_maintenance_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_domains_usage_yyyymm(
            "sample",
            Some("header-value".into()),
            Some(vec!["sample"].into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.get_v2_domains_usage_yyyymm"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .patch_v2_customers_customer_id_domains_domain_contacts(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PATCH");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.patch_v2_customers_customer_id_domains_domain_contacts"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .post_v2_customers_customer_id_domains_domain_regenerate_auth_code(
            "sample",
            "sample",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(
        !request.url.contains('{'),
        "domains.post_v2_customers_customer_id_domains_domain_regenerate_auth_code"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .orders()
        .list(
            "header-value",
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("sample".into()),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "orders.list");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .orders()
        .get(
            "sample",
            "header-value",
            Some("header-value".into()),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "orders.get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .parking()
        .get_metrics(
            "sample",
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "parking.get_metrics");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .parking()
        .get_metrics_by_domain(
            "sample",
            "sample",
            "sample",
            Some(vec!["sample"].into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "parking.get_metrics_by_domain");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .shoppers()
        .create_subaccount(json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "shoppers.create_subaccount");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .shoppers()
        .get(json!({"sample": true}), Some(vec!["sample"].into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "shoppers.get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .shoppers()
        .update(json!({"sample": true}), json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "shoppers.update");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .shoppers()
        .delete(json!({"sample": true}), "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "shoppers.delete");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .shoppers()
        .get_status(json!({"sample": true}), "sample")
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "shoppers.get_status");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .shoppers()
        .change_password(json!({"sample": true}), json!({"sample": true}))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PUT");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "shoppers.change_password");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .subscriptions()
        .list(
            "header-value",
            Some("header-value".into()),
            Some("header-value".into()),
            Some(vec!["sample"].into()),
            Some(vec!["sample"].into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("sample".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "subscriptions.list");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .subscriptions()
        .product_groups("header-value", Some("header-value".into()))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "subscriptions.product_groups");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .subscriptions()
        .cancel(
            json!({"sample": true}),
            "header-value",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "DELETE");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "subscriptions.cancel");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .subscriptions()
        .get(
            json!({"sample": true}),
            "header-value",
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "subscriptions.get");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .subscriptions()
        .update(
            json!({"sample": true}),
            "header-value",
            json!({"sample": true}),
            Some("header-value".into()),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "PATCH");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "subscriptions.update");
}
