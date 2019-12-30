/**
 * Type overrides for "custom scalars" defined by the graphql schema.
 *
 * The types generated from the schema will reference custom scalars without
 * defining what they actually are. We need to define aliases for these types
 * here to complete the typing coverage.
 *
 * The types in this file will be in scope automatically so long as there are
 * no usages of the `import` or `export` keyword.
 *
 *  See: https://stackoverflow.com/a/42257742/140396
 */

/**
 * An ISO Date time.
 */
type E2eDateTimeUtc = string;
