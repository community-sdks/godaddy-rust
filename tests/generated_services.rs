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
        .get_tickets(godaddy_rust::dto::abuse::request::GetTicketsRequest::new(
            Some("sample".into()),
            Some(true.into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        ))
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
        .create_ticket(godaddy_rust::dto::abuse::request::CreateTicketRequest::new(
            json!({"sample": true}),
        ))
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
    client
        .abuse()
        .get_ticket_info(godaddy_rust::dto::abuse::request::GetTicketInfoRequest::new("sample"))
        .await
        .unwrap();
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
        .get_tickets_v2(godaddy_rust::dto::abuse::request::GetTicketsV2Request::new(
            Some("sample".into()),
            Some(true.into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        ))
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
        .create_ticket_v2(
            godaddy_rust::dto::abuse::request::CreateTicketV2Request::new(json!({"sample": true})),
        )
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
    client
        .abuse()
        .get_ticket_info_v2(
            godaddy_rust::dto::abuse::request::GetTicketInfoV2Request::new("sample"),
        )
        .await
        .unwrap();
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
            godaddy_rust::dto::aftermarket::request::GetListingsRequest::new(
                "sample",
                Some(vec!["sample"].into()),
                Some(vec!["sample"].into()),
                Some("sample".into()),
                Some("sample".into()),
                Some(1_i64.into()),
                Some(1_i64.into()),
            ),
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
        .delete_listings(
            godaddy_rust::dto::aftermarket::request::DeleteListingsRequest::new(vec!["sample"]),
        )
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
        .add_expiry_listings(
            godaddy_rust::dto::aftermarket::request::AddExpiryListingsRequest::new(vec!["sample"]),
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
        "aftermarket.add_expiry_listings"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .agreements()
        .get(godaddy_rust::dto::agreements::request::GetRequest::new(
            vec!["sample"],
            Some("header-value".into()),
            Some("header-value".into()),
        ))
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
        .search_ans_name(godaddy_rust::dto::ans::request::SearchAnsNameRequest::new(
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        ))
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
        .register_agent(godaddy_rust::dto::ans::request::RegisterAgentRequest::new(
            json!({"sample": true}),
        ))
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
        .resolve_ans_name(godaddy_rust::dto::ans::request::ResolveAnsNameRequest::new(
            json!({"sample": true}),
        ))
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
    client
        .ans()
        .get_agent(godaddy_rust::dto::ans::request::GetAgentRequest::new(
            "sample",
        ))
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.get_agent");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .validate_registration(
            godaddy_rust::dto::ans::request::ValidateRegistrationRequest::new("sample"),
        )
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "ans.validate_registration");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .verify_dns_records(godaddy_rust::dto::ans::request::VerifyDnsRecordsRequest::new("sample"))
        .await
        .unwrap();
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
        .get_agent_identity_certificate_by_agent_id(
            godaddy_rust::dto::ans::request::GetAgentIdentityCertificateByAgentIdRequest::new(
                "sample",
            ),
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
        "ans.get_agent_identity_certificate_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .submit_agent_identity_csr_by_agent_id(
            godaddy_rust::dto::ans::request::SubmitAgentIdentityCsrByAgentIdRequest::new(
                "sample",
                json!({"sample": true}),
            ),
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
        "ans.submit_agent_identity_csr_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .get_agent_server_certificate_by_agent_id(
            godaddy_rust::dto::ans::request::GetAgentServerCertificateByAgentIdRequest::new(
                "sample",
            ),
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
        "ans.get_agent_server_certificate_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .submit_agent_server_csr_by_agent_id(
            godaddy_rust::dto::ans::request::SubmitAgentServerCsrByAgentIdRequest::new(
                "sample",
                json!({"sample": true}),
            ),
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
        "ans.submit_agent_server_csr_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .get_agent_csr_status_by_agent_id(
            godaddy_rust::dto::ans::request::GetAgentCsrStatusByAgentIdRequest::new(
                "sample", "sample",
            ),
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
        "ans.get_agent_csr_status_by_agent_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .ans()
        .get_agent_events(godaddy_rust::dto::ans::request::GetAgentEventsRequest::new(
            Some("header-value".into()),
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
        ))
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
        .place_bids(godaddy_rust::dto::auctions::request::PlaceBidsRequest::new(
            "sample",
            json!({"sample": true}),
        ))
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
        .certificate_create(
            godaddy_rust::dto::certificates::request::CertificateCreateRequest::new(
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        "certificates.certificate_create"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_validate(
            godaddy_rust::dto::certificates::request::CertificateValidateRequest::new(
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        "certificates.certificate_validate"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_get(
            godaddy_rust::dto::certificates::request::CertificateGetRequest::new("sample"),
        )
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
        .certificate_action_retrieve(
            godaddy_rust::dto::certificates::request::CertificateActionRetrieveRequest::new(
                "sample",
            ),
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
        "certificates.certificate_action_retrieve"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_resend_email(
            godaddy_rust::dto::certificates::request::CertificateResendEmailRequest::new(
                "sample", "sample",
            ),
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
        "certificates.certificate_resend_email"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_alternate_email_address(
            godaddy_rust::dto::certificates::request::CertificateAlternateEmailAddressRequest::new(
                "sample",
                vec!["sample"],
            ),
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
        "certificates.certificate_alternate_email_address"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_resend_email_address(
            godaddy_rust::dto::certificates::request::CertificateResendEmailAddressRequest::new(
                "sample",
                "sample",
                vec!["sample"],
            ),
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
        "certificates.certificate_resend_email_address"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_email_history(
            godaddy_rust::dto::certificates::request::CertificateEmailHistoryRequest::new("sample"),
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
        "certificates.certificate_email_history"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_callback_delete(
            godaddy_rust::dto::certificates::request::CertificateCallbackDeleteRequest::new(
                "sample",
            ),
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
        "certificates.certificate_callback_delete"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_callback_get(
            godaddy_rust::dto::certificates::request::CertificateCallbackGetRequest::new("sample"),
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
        "certificates.certificate_callback_get"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_callback_replace(
            godaddy_rust::dto::certificates::request::CertificateCallbackReplaceRequest::new(
                "sample", "sample",
            ),
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
        "certificates.certificate_callback_replace"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_cancel(
            godaddy_rust::dto::certificates::request::CertificateCancelRequest::new("sample"),
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
        "certificates.certificate_cancel"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_download(
            godaddy_rust::dto::certificates::request::CertificateDownloadRequest::new("sample"),
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
        "certificates.certificate_download"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_reissue(
            godaddy_rust::dto::certificates::request::CertificateReissueRequest::new(
                "sample",
                json!({"sample": true}),
            ),
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
        "certificates.certificate_reissue"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_renew(
            godaddy_rust::dto::certificates::request::CertificateRenewRequest::new(
                "sample",
                json!({"sample": true}),
            ),
        )
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
        .certificate_revoke(
            godaddy_rust::dto::certificates::request::CertificateRevokeRequest::new(
                "sample",
                json!({"sample": true}),
            ),
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
        "certificates.certificate_revoke"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_siteseal_get(
            godaddy_rust::dto::certificates::request::CertificateSitesealGetRequest::new(
                "sample",
                Some("sample".into()),
                Some("sample".into()),
            ),
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
        "certificates.certificate_siteseal_get"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_verifydomaincontrol(
            godaddy_rust::dto::certificates::request::CertificateVerifydomaincontrolRequest::new(
                "sample",
            ),
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
        "certificates.certificate_verifydomaincontrol"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_get_entitlement(
            godaddy_rust::dto::certificates::request::CertificateGetEntitlementRequest::new(
                "sample",
                Some(true.into()),
            ),
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
        "certificates.certificate_get_entitlement"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_create_v2(
            godaddy_rust::dto::certificates::request::CertificateCreateV2Request::new(
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        "certificates.certificate_create_v2"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .certificate_download_entitlement(
            godaddy_rust::dto::certificates::request::CertificateDownloadEntitlementRequest::new(
                "sample",
            ),
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
        "certificates.certificate_download_entitlement"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .get_customer_certificates_by_customer_id(godaddy_rust::dto::certificates::request::GetCustomerCertificatesByCustomerIdRequest::new(
            "sample", Some(1_i64.into()), Some(1_i64.into())
        ))
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
        .get_certificate_detail_by_cert_identifier(godaddy_rust::dto::certificates::request::GetCertificateDetailByCertIdentifierRequest::new(
            "sample", "sample"
        ))
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
        .get_domain_information_by_certificate_id(godaddy_rust::dto::certificates::request::GetDomainInformationByCertificateIdRequest::new(
            "sample", "sample"
        ))
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
        .get_domain_details_by_domain(
            godaddy_rust::dto::certificates::request::GetDomainDetailsByDomainRequest::new(
                "sample", "sample", "sample",
            ),
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
        "certificates.get_domain_details_by_domain"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .get_acme_external_account_binding(
            godaddy_rust::dto::certificates::request::GetAcmeExternalAccountBindingRequest::new(
                "sample",
            ),
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
        "certificates.get_acme_external_account_binding"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .certificates()
        .retrieve_ssl_by_domain_reseller(
            godaddy_rust::dto::certificates::request::RetrieveSslByDomainResellerRequest::new(
                Some(1_i64.into()),
                Some(1_i64.into()),
                Some("sample".into()),
                Some(vec!["sample"].into()),
                Some("sample".into()),
                Some("sample".into()),
            ),
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
        .retrieve_ssl_by_domain_subscription_reseller(godaddy_rust::dto::certificates::request::RetrieveSslByDomainSubscriptionResellerRequest::new(
            "sample",
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("sample".into()),
            Some(vec!["sample"].into()),
            Some("sample".into()),
            Some("sample".into()),
        ))
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
    client
        .countries()
        .get_countries(godaddy_rust::dto::countries::request::GetCountriesRequest::new("sample"))
        .await
        .unwrap();
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
        .get_country(
            godaddy_rust::dto::countries::request::GetCountryRequest::new("sample", "sample"),
        )
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
        .list(godaddy_rust::dto::domains::request::ListRequest::new(
            Some("header-value".into()),
            Some(vec!["sample"].into()),
            Some(vec!["sample"].into()),
            Some(1_i64.into()),
            Some("sample".into()),
            Some(vec!["sample"].into()),
            Some("sample".into()),
        ))
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
            godaddy_rust::dto::domains::request::GetAgreementRequest::new(
                vec!["sample"],
                true,
                Some("header-value".into()),
                Some(true.into()),
            ),
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
        .available(godaddy_rust::dto::domains::request::AvailableRequest::new(
            "sample",
            Some("sample".into()),
            Some(true.into()),
        ))
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
        .available_bulk(
            godaddy_rust::dto::domains::request::AvailableBulkRequest::new(
                vec!["sample"],
                Some("sample".into()),
            ),
        )
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
            godaddy_rust::dto::domains::request::ContactsValidateRequest::new(
                json!({"sample": true}),
                Some("header-value".into()),
                Some("sample".into()),
            ),
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
        .purchase(godaddy_rust::dto::domains::request::PurchaseRequest::new(
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
    client
        .domains()
        .schema(godaddy_rust::dto::domains::request::SchemaRequest::new(
            "sample",
        ))
        .await
        .unwrap();
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
        .validate(godaddy_rust::dto::domains::request::ValidateRequest::new(
            json!({"sample": true}),
        ))
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
        .suggest(godaddy_rust::dto::domains::request::SuggestRequest::new(
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
        ))
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
    client
        .domains()
        .tlds(godaddy_rust::dto::domains::request::TldsRequest::new())
        .await
        .unwrap();
    let request = transport.request_at(before).expect("request recorded");
    assert_eq!(request.method, "GET");
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("sso-key key:secret")
    );
    assert!(!request.url.contains('{'), "domains.tlds");

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .cancel(godaddy_rust::dto::domains::request::CancelRequest::new(
            "sample",
        ))
        .await
        .unwrap();
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
        .get(godaddy_rust::dto::domains::request::GetRequest::new(
            "sample",
            Some("header-value".into()),
        ))
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
        .update(godaddy_rust::dto::domains::request::UpdateRequest::new(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
            godaddy_rust::dto::domains::request::UpdateContactsRequest::new(
                "sample",
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        .cancel_privacy(
            godaddy_rust::dto::domains::request::CancelPrivacyRequest::new(
                "sample",
                Some("header-value".into()),
            ),
        )
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
            godaddy_rust::dto::domains::request::PurchasePrivacyRequest::new(
                "sample",
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        .record_add(godaddy_rust::dto::domains::request::RecordAddRequest::new(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
            godaddy_rust::dto::domains::request::RecordReplaceRequest::new(
                "sample",
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        .record_get(godaddy_rust::dto::domains::request::RecordGetRequest::new(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
        ))
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
            godaddy_rust::dto::domains::request::RecordReplaceTypeNameRequest::new(
                "sample",
                "sample",
                "sample",
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        .record_delete_type_name(
            godaddy_rust::dto::domains::request::RecordDeleteTypeNameRequest::new(
                "sample",
                "sample",
                "sample",
                Some("header-value".into()),
            ),
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
        "domains.record_delete_type_name"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .record_replace_type(
            godaddy_rust::dto::domains::request::RecordReplaceTypeRequest::new(
                "sample",
                "sample",
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
        .renew(godaddy_rust::dto::domains::request::RenewRequest::new(
            "sample",
            Some("header-value".into()),
            Some(json!({"sample": true}).into()),
        ))
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
        .transfer_in(godaddy_rust::dto::domains::request::TransferInRequest::new(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .verify_email(
            godaddy_rust::dto::domains::request::VerifyEmailRequest::new(
                "sample",
                Some("header-value".into()),
            ),
        )
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
            godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainRequest::new(
                "sample",
                "sample",
                Some("header-value".into()),
                Some(vec!["sample"].into()),
            ),
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
        .delete_v2_customers_customer_id_domains_domain_change_of_registrant(godaddy_rust::dto::domains::request::DeleteV2CustomersCustomerIdDomainsDomainChangeOfRegistrantRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .get_v2_customers_customer_id_domains_domain_change_of_registrant(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainChangeOfRegistrantRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .patch_v2_customers_customer_id_domains_domain_dnssec_records(godaddy_rust::dto::domains::request::PatchV2CustomersCustomerIdDomainsDomainDnssecRecordsRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .delete_v2_customers_customer_id_domains_domain_dnssec_records(godaddy_rust::dto::domains::request::DeleteV2CustomersCustomerIdDomainsDomainDnssecRecordsRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .put_v2_customers_customer_id_domains_domain_name_servers(godaddy_rust::dto::domains::request::PutV2CustomersCustomerIdDomainsDomainNameServersRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .get_v2_customers_customer_id_domains_domain_privacy_forwarding(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainPrivacyForwardingRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .patch_v2_customers_customer_id_domains_domain_privacy_forwarding(godaddy_rust::dto::domains::request::PatchV2CustomersCustomerIdDomainsDomainPrivacyForwardingRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_redeem(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainRedeemRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
            Some(json!({"sample": true}).into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_renew(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainRenewRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .get_v2_customers_customer_id_domains_domain_transfer(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainTransferRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_validate(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferValidateRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_in_accept(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInAcceptRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_in_cancel(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInCancelRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_in_restart(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInRestartRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_in_retry(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferInRetryRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_out(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferOutRequest::new(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_out_accept(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferOutAcceptRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_transfer_out_reject(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainTransferOutRejectRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
            Some("sample".into()),
        ))
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
        .domains_forwards_delete(
            godaddy_rust::dto::domains::request::DomainsForwardsDeleteRequest::new(
                "sample", "sample",
            ),
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
        "domains.domains_forwards_delete"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .domains_forwards_get(
            godaddy_rust::dto::domains::request::DomainsForwardsGetRequest::new(
                "sample",
                "sample",
                Some(true.into()),
            ),
        )
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
        .domains_forwards_put(
            godaddy_rust::dto::domains::request::DomainsForwardsPutRequest::new(
                "sample",
                "sample",
                json!({"sample": true}),
            ),
        )
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
        .domains_forwards_post(
            godaddy_rust::dto::domains::request::DomainsForwardsPostRequest::new(
                "sample",
                "sample",
                json!({"sample": true}),
            ),
        )
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
        .get_v2_customers_customer_id_domains_domain_actions(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainActionsRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .delete_v2_customers_customer_id_domains_domain_actions_type(godaddy_rust::dto::domains::request::DeleteV2CustomersCustomerIdDomainsDomainActionsTypeRequest::new(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .get_v2_customers_customer_id_domains_domain_actions_type(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsDomainActionsTypeRequest::new(
            "sample",
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .get_v2_customers_customer_id_domains_notifications(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsNotificationsRequest::new(
            "sample", Some("header-value".into())
        ))
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
        .get_v2_customers_customer_id_domains_notifications_opt_in(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsNotificationsOptInRequest::new(
            "sample",
            Some("header-value".into()),
        ))
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
        .put_v2_customers_customer_id_domains_notifications_opt_in(godaddy_rust::dto::domains::request::PutV2CustomersCustomerIdDomainsNotificationsOptInRequest::new(
            "sample",
            vec!["sample"],
            Some("header-value".into()),
        ))
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
        .get_v2_customers_customer_id_domains_notifications_schemas_type(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsNotificationsSchemasTypeRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_notifications_notification_id_acknowledge(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsNotificationsNotificationIdAcknowledgeRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_register(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsRegisterRequest::new(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .get_v2_customers_customer_id_domains_register_schema_tld(godaddy_rust::dto::domains::request::GetV2CustomersCustomerIdDomainsRegisterSchemaTldRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_register_validate(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsRegisterValidateRequest::new(
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
            godaddy_rust::dto::domains::request::GetV2DomainsMaintenancesRequest::new(
                Some("header-value".into()),
                Some(vec!["sample"].into()),
                Some("sample".into()),
                Some("sample".into()),
                Some(1_i64.into()),
            ),
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
        .get_v2_domains_maintenances_maintenance_id(
            godaddy_rust::dto::domains::request::GetV2DomainsMaintenancesMaintenanceIdRequest::new(
                "sample",
                Some("header-value".into()),
            ),
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
        "domains.get_v2_domains_maintenances_maintenance_id"
    );

    transport.push_response(Response::json(200, "{}"));
    let before = transport.request_count();
    client
        .domains()
        .get_v2_domains_usage_yyyymm(
            godaddy_rust::dto::domains::request::GetV2DomainsUsageYyyymmRequest::new(
                "sample",
                Some("header-value".into()),
                Some(vec!["sample"].into()),
            ),
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
        .patch_v2_customers_customer_id_domains_domain_contacts(godaddy_rust::dto::domains::request::PatchV2CustomersCustomerIdDomainsDomainContactsRequest::new(
            "sample",
            "sample",
            json!({"sample": true}),
            Some("header-value".into()),
        ))
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
        .post_v2_customers_customer_id_domains_domain_regenerate_auth_code(godaddy_rust::dto::domains::request::PostV2CustomersCustomerIdDomainsDomainRegenerateAuthCodeRequest::new(
            "sample",
            "sample",
            Some("header-value".into()),
        ))
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
        .list(godaddy_rust::dto::orders::request::ListRequest::new(
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
        ))
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
        .get(godaddy_rust::dto::orders::request::GetRequest::new(
            "sample",
            "header-value",
            Some("header-value".into()),
            Some("header-value".into()),
        ))
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
        .get_metrics(godaddy_rust::dto::parking::request::GetMetricsRequest::new(
            "sample",
            Some("sample".into()),
            Some("sample".into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("header-value".into()),
        ))
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
            godaddy_rust::dto::parking::request::GetMetricsByDomainRequest::new(
                "sample",
                "sample",
                "sample",
                Some(vec!["sample"].into()),
                Some("sample".into()),
                Some("sample".into()),
                Some(1_i64.into()),
                Some(1_i64.into()),
                Some("header-value".into()),
            ),
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
        .create_subaccount(
            godaddy_rust::dto::shoppers::request::CreateSubaccountRequest::new(
                json!({"sample": true}),
            ),
        )
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
        .get(godaddy_rust::dto::shoppers::request::GetRequest::new(
            json!({"sample": true}),
            Some(vec!["sample"].into()),
        ))
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
        .update(godaddy_rust::dto::shoppers::request::UpdateRequest::new(
            json!({"sample": true}),
            json!({"sample": true}),
        ))
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
        .delete(godaddy_rust::dto::shoppers::request::DeleteRequest::new(
            json!({"sample": true}),
            "sample",
        ))
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
        .get_status(godaddy_rust::dto::shoppers::request::GetStatusRequest::new(
            json!({"sample": true}),
            "sample",
        ))
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
        .change_password(
            godaddy_rust::dto::shoppers::request::ChangePasswordRequest::new(
                json!({"sample": true}),
                json!({"sample": true}),
            ),
        )
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
        .list(godaddy_rust::dto::subscriptions::request::ListRequest::new(
            "header-value",
            Some("header-value".into()),
            Some("header-value".into()),
            Some(vec!["sample"].into()),
            Some(vec!["sample"].into()),
            Some(1_i64.into()),
            Some(1_i64.into()),
            Some("sample".into()),
        ))
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
        .product_groups(
            godaddy_rust::dto::subscriptions::request::ProductGroupsRequest::new(
                "header-value",
                Some("header-value".into()),
            ),
        )
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
            godaddy_rust::dto::subscriptions::request::CancelRequest::new(
                json!({"sample": true}),
                "header-value",
                Some("header-value".into()),
            ),
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
        .get(godaddy_rust::dto::subscriptions::request::GetRequest::new(
            json!({"sample": true}),
            "header-value",
            Some("header-value".into()),
        ))
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
            godaddy_rust::dto::subscriptions::request::UpdateRequest::new(
                json!({"sample": true}),
                "header-value",
                json!({"sample": true}),
                Some("header-value".into()),
            ),
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
