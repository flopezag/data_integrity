use utoipa::OpenApi;
use crate::handlers::{version, sign, verify};

#[derive(OpenApi)]
#[openapi(
    paths(
        version::service_info,
        sign::sign_handler,
        verify::verify_handler
    ),
    components(
        schemas(
            version::ServiceInfo,
            sign::SignRequest,
            verify::VerifyRequest,
            verify::VerifyResult,
            verify::VerificationStatus
        )
    ),
    tags(
        (name = "NGSI-LD API", description = "NGSI-LD signing and verification API")
    )
)]

pub struct ApiDoc;
