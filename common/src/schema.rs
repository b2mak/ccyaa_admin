// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Int4,
        squarespace_id -> Varchar,
        order_number -> Varchar,
        created_on -> Varchar,
        modified_on -> Varchar,
        channel -> Varchar,
        testmode -> Bool,
        customer_email -> Varchar,
        billing_address -> Jsonb,
        shipping_address -> Nullable<Jsonb>,
        fulfillment_status -> Varchar,
        line_items -> Array<Jsonb>,
        internal_notes -> Array<Jsonb>,
        shipping_lines -> Array<Jsonb>,
        discount_lines -> Array<Jsonb>,
        form_submission -> Nullable<Array<Jsonb>>,
        fulfillments -> Array<Jsonb>,
        subtotal -> Jsonb,
        shipping_total -> Jsonb,
        discount_total -> Jsonb,
        tax_total -> Jsonb,
        refunded_total -> Jsonb,
        grand_total -> Jsonb,
        channel_name -> Varchar,
        external_order_reference -> Nullable<Varchar>,
        fulfilled_on -> Nullable<Varchar>,
        price_tax_interpretation -> Nullable<Varchar>,
    }
}
