pub(crate) struct SharedAccessSignature {

}


impl SharedAccessSignature {
    /// <summary>The maximum allowed length of the SAS key name.</summary>
    internal const int MaximumKeyNameLength = 256;

    /// <summary>The maximum allowed length of the SAS key.</summary>
    private const int MaximumKeyLength = 256;

    /// <summary>The token that represents the type of authentication used.</summary>
    private const string AuthenticationTypeToken = "SharedAccessSignature";

    /// <summary>The token that identifies the signed component of the shared access signature.</summary>
    private const string SignedResourceToken = "sr";

    /// <summary>The token that identifies the signature component of the shared access signature.</summary>
    private const string SignatureToken = "sig";

    /// <summary>The token that identifies the signed SAS key component of the shared access signature.</summary>
    private const string SignedKeyNameToken = "skn";

    /// <summary>The token that identifies the signed expiration time of the shared access signature.</summary>
    private const string SignedExpiryToken = "se";

    /// <summary>The token that fully identifies the signed resource within the signature.</summary>
    private const string SignedResourceFullIdentifierToken = AuthenticationTypeToken + " " + SignedResourceToken;

    /// <summary>The character used to separate a token and its value in the connection string.</summary>
    private const char TokenValueSeparator = '=';

    /// <summary>The character used to mark the beginning of a new token/value pair in the signature.</summary>
    private const char TokenValuePairDelimiter = '&';

    /// <summary>The default length of time to consider a signature valid, if not otherwise specified.</summary>
    private static readonly TimeSpan DefaultSignatureValidityDuration = TimeSpan.FromMinutes(30);

    /// <summary>Represents the Unix epoch time value, January 1, 1970 12:00:00, UTC.</summary>
    private static readonly DateTimeOffset Epoch = new DateTimeOffset(1970, 1, 1, 0, 0, 0, TimeSpan.Zero);
}
