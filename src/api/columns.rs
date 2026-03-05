use std::collections::HashMap;
use std::sync::LazyLock;

/// Maps Semrush cryptic column codes to human-readable field names.
static COLUMN_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        // Keyword / Phrase columns
        ("Ph", "keyword"),
        ("Nq", "search_volume"),
        ("Cp", "cpc"),
        ("Co", "competition"),
        ("Kd", "keyword_difficulty"),
        ("Nr", "results_count"),
        ("Td", "trends"),
        ("In", "intent"),
        ("Fk", "featured_keyword"),
        // Domain columns
        ("Dn", "domain"),
        ("Rk", "rank"),
        ("Or", "organic_keywords"),
        ("Ot", "organic_traffic"),
        ("Oc", "organic_cost"),
        ("Ad", "paid_keywords"),
        ("At", "paid_traffic"),
        ("Ac", "paid_cost"),
        ("Sh", "pla_keywords"),
        ("Sv", "pla_uniques"),
        ("FKn", "featured_snippet_keywords"),
        // Position columns
        ("Po", "position"),
        ("Pp", "previous_position"),
        ("Pd", "position_difference"),
        ("Ur", "url"),
        ("Tr", "traffic_percent"),
        ("Tc", "traffic_cost"),
        ("Tg", "timestamp"),
        // Backlink columns
        ("source_url", "source_url"),
        ("source_title", "source_title"),
        ("target_url", "target_url"),
        ("anchor", "anchor"),
        ("external_num", "external_links"),
        ("internal_num", "internal_links"),
        ("first_seen", "first_seen"),
        ("last_seen", "last_seen"),
        ("page_score", "page_authority"),
        ("nofollow", "nofollow"),
        ("form", "form"),
        ("frame", "frame"),
        ("image", "image"),
        ("sitewide", "sitewide"),
        ("newlink", "new_link"),
        ("lostlink", "lost_link"),
        ("response_code", "response_code"),
        ("backlinks_num", "total_backlinks"),
        ("domains_num", "referring_domains"),
        ("ips_num", "referring_ips"),
        ("follows_num", "follow_links"),
        ("nofollows_num", "nofollow_links"),
        ("texts_num", "text_links"),
        ("images_num", "image_links"),
        ("forms_num", "form_links"),
        ("frames_num", "frame_links"),
        ("score", "authority_score"),
        // Domain comparison
        ("Np", "common_keywords"),
        ("Nm", "missing_keywords"),
        // Ads
        ("Tt", "ad_title"),
        ("Ds", "ad_description"),
        ("Vu", "visible_url"),
        ("Dt", "date"),
        // PLA / Shopping
        ("St", "product_title"),
        ("Sp", "product_price"),
        ("Sn", "shop_name"),
        // Traffic / Trends
        ("visits", "visits"),
        ("users", "unique_visitors"),
        ("pages_per_visit", "pages_per_visit"),
        ("bounce_rate", "bounce_rate"),
        ("avg_visit_duration", "avg_visit_duration"),
        // Overview
        ("Db", "database"),
    ])
});

/// Convert a Semrush column code to its human-readable name.
/// Returns the original code if no mapping exists.
pub fn to_human(code: &str) -> &str {
    COLUMN_MAP.get(code).copied().unwrap_or(code)
}

/// Convert a human-readable name back to Semrush column code.
/// Returns the original name if no reverse mapping exists.
pub fn to_code(human: &str) -> &str {
    COLUMN_MAP
        .iter()
        .find(|(_, v)| **v == human)
        .map(|(k, _)| *k)
        .unwrap_or(human)
}

/// Get the default export columns for a report type.
pub fn default_columns(report_type: &str) -> &'static str {
    match report_type {
        "domain_rank" => "Db,Dn,Rk,Or,Ot,Oc,Ad,At,Ac,Sh,Sv",
        "domain_ranks" => "Db,Dn,Rk,Or,Ot,Oc,Ad,At,Ac",
        "domain_rank_history" => "Rk,Or,Ot,Oc,Ad,At,Ac,Dt",
        "domain_organic" => "Ph,Po,Pp,Nq,Cp,Ur,Tr,Tc,Co,Kd",
        "domain_adwords" => "Ph,Po,Nq,Cp,Ur,Tr,Tc,Co,Kd",
        "domain_adwords_unique" => "Tt,Ds,Vu,Ph,Po,Ur",
        "domain_adwords_historical" => "Ph,Po,Ur,Dt",
        "domain_organic_organic" => "Dn,Np,Or,Ot,Oc,Ad",
        "domain_adwords_adwords" => "Dn,Np,Ad,At,Ac,Or",
        "domain_shopping" => "Ph,Po,Nq,Cp,Ur,Tr",
        "domain_shopping_unique" => "St,Sp,Sn,Ur",
        "domain_shopping_shopping" => "Dn,Np,Sh,Sv",
        "domain_organic_unique" => "Ur,Tr,Tc",
        "domain_organic_subdomains" => "Dn,Or,Ot,Oc",
        "domain_domains" => "Ph,Nq,Kd,Co,Dn",
        "phrase_this" => "Ph,Nq,Cp,Co,Nr,Td,Kd,In",
        "phrase_all" => "Db,Ph,Nq,Cp,Co,Nr,Kd",
        "phrase_these" => "Ph,Nq,Cp,Co,Nr,Td,Kd,In",
        "phrase_organic" => "Dn,Ur,Po,Tr,Tc",
        "phrase_adwords" => "Dn,Ur,Po,Tr,Tc",
        "phrase_related" => "Ph,Nq,Cp,Co,Nr,Td,Kd,In",
        "phrase_fullsearch" => "Ph,Nq,Cp,Co,Nr,Td,Kd,In",
        "phrase_questions" => "Ph,Nq,Cp,Co,Nr,Td,Kd,In",
        "phrase_kdi" => "Ph,Kd",
        "phrase_adwords_historical" => "Ph,Dt,Po,Ur",
        "rank" => "Dn,Rk,Or,Ot,Oc,Ad,At,Ac",
        "rank_difference" => "Dn,Rk,Or,Ot,Oc,Ad,At,Ac",
        _ => "",
    }
}
