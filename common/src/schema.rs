// @generated automatically by Diesel CLI.

diesel::table! {
    line_items (id) {
        id -> Varchar,
        order_id -> Varchar,
        variant_id -> Nullable<Varchar>,
        sku -> Varchar,
        weight -> Float4,
        width -> Float4,
        length -> Float4,
        height -> Float4,
        product_id -> Nullable<Varchar>,
        product_name -> Nullable<Varchar>,
        quantity -> Int4,
        unit_price_paid -> Jsonb,
        variant_options -> Array<Jsonb>,
        customizations -> Nullable<Array<Jsonb>>,
        image_url -> Varchar,
        line_item_type -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Varchar,
        order_number -> Varchar,
        created_on -> Varchar,
        modified_on -> Varchar,
        channel -> Varchar,
        testmode -> Bool,
        customer_email -> Varchar,
        billing_address -> Jsonb,
        shipping_address -> Nullable<Jsonb>,
        fulfillment_status -> Varchar,
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

diesel::joinable!(line_items -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    line_items,
    orders,
);
