# TlsCertificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | **String** | The negotiated SSL/TLS protocol version.  | 
**cipher_name** | **String** | The OpenSSL name of the cipher suite.  | 
**authorized** | **bool** | Indicates whether a trusted authority signed the certificate.  | 
**error** | Option<**String**> | The reason for rejecting the certificate if `authorized` is `false`.  | [optional]
**created_at** | **String** | The creation date and time of the certificate. | 
**expires_at** | **String** | The expiration date and time of the certificate. | 
**subject** | [**models::TlsCertificateSubject**](TlsCertificateSubject.md) |  | 
**issuer** | [**models::TlsCertificateIssuer**](TlsCertificateIssuer.md) |  | 
**key_type** | Option<**String**> | The type of the used key, or `null` for unrecognized types. | 
**key_bits** | Option<**f64**> | The size of the used key, or `null` for unrecognized types. | 
**serial_number** | **String** | The certificate serial number as a : separated HEX string.  | 
**fingerprint256** | **String** | The SHA-256 digest of the DER-encoded certificate as a : separated HEX string.  | 
**public_key** | Option<**String**> | The public key as a : separated HEX string, or `null` for unrecognized types.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


