use utoipa::OpenApi;
use crate::handlers::{version, sign, verify, config};

#[derive(OpenApi)]
#[openapi(
    paths(
        version::service_info,
        sign::sign_handler,
        verify::verify_handler,
        config::config_handler
    ),
    components(
        schemas(
            version::ServiceInfo,
            verify::VerifyRequest,
            verify::VerifyResult,
            verify::VerificationStatus,
            config::ConfigRequest
        )
    ),
    tags(
        (
            name = "Node Data Manager API", 
            description = "NGSI-LD signing and verification API compliant with 
            ETSI Handling of Provenance Information in NGSI-LDD specification 
            (ETSI GS CIM 019 V1.1.2) and ETSI NGSI-LD API specification 
            (ETSI GS CIM 009 V1.9.1)")
    )
)]

pub struct ApiDoc;
