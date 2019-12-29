pub struct Headers;

impl Headers {
    pub const HOST: &'static str = "host";
    pub const IF_MATCH: &'static str = "if-match";
    pub const IF_MODIFIED_SINCE: &'static str = "if-modified-since";
    pub const IF_NONE_MATCH: &'static str = "if-none-match";
    pub const IF_UNMODIFIED_SINCE: &'static str = "if-unmodified-since";
    pub const PART_NUMBER: &'static str = "part-number";
    pub const RANGE: &'static str = "range";
    pub const REQUEST_PAYER: &'static str = "request-payer";
    pub const RESPONSE_CACHE_CONTROL: &'static str = "response-cache-control";
    pub const RESPONSE_CONTENT_DISPOSITION: &'static str = "response-content-disposition";
    pub const RESPONSE_CONTENT_ENCODING: &'static str = "response-content-encoding";
    pub const RESPONSE_CONTENT_LANGUAGE: &'static str = "responsee-content-language";
    pub const RESPONSE_CONTENT_TYPE: &'static str = "response-content-type";
    pub const RESPONSE_EXPIRES: &'static str = "response-expires";
    pub const SSE_CUSTOMER_ALGORITHM: &'static str = "sse-customer-algorithm";
    pub const SSE_CUSTOMER_KEY: &'static str = "sse-customer-key";
    pub const SSE_CUSTOMER_KEY_MD5: &'static str = "sse-customer-key-md5";
    pub const VERSION_ID: &'static str = "vesrion-id";
    pub const X_AMZ_CONTENT_SHA256: &'static str = "x-amz-content-sha256";
    pub const X_AMZ_DATE: &'static str = "x-amz-date";

    pub const AUTHORIZATION: &'static str = "authorization";
    pub const DELETE_MARKER: &'static str = "x-amz-delete-marker";
    pub const LAST_MODIFIED: &'static str = "last-modified";
    pub const ETAG: &'static str = "etag";
    pub const EXPIRES: &'static str = "expires";
    pub const PARTS_COUNT: &'static str = "x-amz-mp-parts-count";
    pub const X_AMZ_VERSION_ID: &'static str = "x-amz-version-id";
    pub const X_AMZ_STORAGE_CLASS: &'static str = "x-amz-storage-class";

    pub const CONTENT_MD5: &'static str = "content-md5";
}
