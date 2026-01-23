# Issue: Create CosmosStatus Type for Combined HTTP Status and SubStatus Codes

## Summary

Create a `CosmosStatus` type that combines HTTP status code with Cosmos DB sub-status code to provide more descriptive string representations that disambiguate reused sub-status codes.

## Background

The `SubStatusCode` type currently only displays the numeric sub-status code value (e.g., `1008`). However, some sub-status codes have different meanings depending on the associated HTTP status code:

- `403.1008` = Database Account Not Found (alias: `DATABASE_ACCOUNT_NOT_FOUND`)
- `410.1008` = Completing Partition Migration (base: `COMPLETING_PARTITION_MIGRATION`)

The current implementation cannot distinguish between these cases when formatting for display or debugging purposes.

## Proposal

Create a `CosmosStatus` type that:

1. **Structure**: Contains both HTTP status code and sub-status code
   ```rust
   pub struct CosmosStatus {
       http_status: u16,
       sub_status: SubStatusCode,
   }
   ```

2. **Display Implementation**: Provides context-aware formatting
   ```rust
   // Examples:
   // "403.1008 (DATABASE_ACCOUNT_NOT_FOUND)"
   // "410.1008 (COMPLETING_PARTITION_MIGRATION)"
   // "429.3200 (RU_BUDGET_EXCEEDED)"
   // "500.9999" (unknown sub-status)
   ```

3. **Benefits**:
   - Disambiguates reused sub-status codes based on HTTP status
   - Provides more meaningful error messages for debugging
   - Maintains the existing `SubStatusCode` type for cases where only the sub-status is available
   - Better aligns with how Cosmos DB errors are actually structured

## Implementation Details

- The `SubStatusCode` type remains as-is (numeric display only)
- `CosmosStatus` becomes the primary type for error reporting
- Migration path: Existing code using `SubStatusCode` continues to work
- New error handling code should use `CosmosStatus` when both codes are available

## References

- Related PR: Add RegionName and SubStatusCode newtypes to Cosmos SDK
- Comment thread: https://github.com/Azure/azure-sdk-for-rust/pull/[PR_NUMBER]/files/[COMMIT_HASH]#r2722826021
