use simdjson_rust::{
    constants::SIMDJSON_MAXSIZE_BYTES, ondemand::parser::Parser, padded_string::PaddedString,
};

fn main() {
    let data = r#"{"ingredients": [{"rose_wine": {"black_sesame_seed": 3}}, {"myrtleberry": {"black_sesame_seed": 5}}, {"grilled_beef": {"black_sesame_seed": 1}}, {"parmesan_cheese": {"black_sesame_seed": 2}}, {"strawberry_jam": {"black_sesame_seed": 1}}, {"tuna": {"black_sesame_seed": 3}}, {"black_tea": {"black_sesame_seed": 8}}, {"japanese_mint": {"black_sesame_seed": 5}}, {"pork": {"black_sesame_seed": 1}}, {"french_lavender": {"black_sesame_seed": 2}}, {"nutmeg": {"black_sesame_seed": 12}}, {"carob_fruit": {"black_sesame_seed": 2}}, {"mastic_gum_leaf_oil": {"black_sesame_seed": 2}}, {"california_pepper": {"black_sesame_seed": 5}}, {"orange_peel_oil": {"black_sesame_seed": 2}}, {"eucalyptus_oil": {"black_sesame_seed": 4}}, {"monarda_punctata": {"black_sesame_seed": 1}}, {"dried_green_tea": {"black_sesame_seed": 7}}, {"sake": {"black_sesame_seed": 2}}, {"currant": {"black_sesame_seed": 2}}, {"sweet_grass_oil": {"black_sesame_seed": 2}}, {"safflower_seed": {"black_sesame_seed": 17}}, {"sparkling_wine": {"black_sesame_seed": 3}}, {"enokidake": {"black_sesame_seed": 1}}, {"raw_peanut": {"black_sesame_seed": 1}}, {"cucumber": {"black_sesame_seed": 1}}, {"russian_cheese": {"black_sesame_seed": 1}}, {"catfish": {"black_sesame_seed": 3}}, {"origanum": {"black_sesame_seed": 2}}, {"roasted_beef": {"black_sesame_seed": 1}}, {"limburger_cheese": {"black_sesame_seed": 1}}, {"cantaloupe": {"black_sesame_seed": 3}}, {"corn_mint_oil": {"black_sesame_seed": 5}}, {"marjoram": {"black_sesame_seed": 4}}, {"herring": {"black_sesame_seed": 3}}, {"kumquat_peel_oil": {"black_sesame_seed": 1}}, {"cheese": {"black_sesame_seed": 1}}, {"lavender": {"black_sesame_seed": 1}}, {"cayenne": {"black_sesame_seed": 5}}, {"red_kidney_bean": {"black_sesame_seed": 4}}, {"mangosteen": {"black_sesame_seed": 2}}, {"lovage_root": {"black_sesame_seed": 1}}, {"mastic_gum": {"black_sesame_seed": 2}}, {"liver": {"black_sesame_seed": 1}}, {"thyme": {"black_sesame_seed": 3}}, {"oregano": {"black_sesame_seed": 1}}, {"lamb_liver": {"black_sesame_seed": 1}}, {"fruit": {"black_sesame_seed": 2}}, {"crab": {"black_sesame_seed": 1}}, {"french_peppermint": {"black_sesame_seed": 7}}, {"jamaican_rum": {"black_sesame_seed": 4}}, {"orange": {"black_sesame_seed": 7}}, {"romano_cheese": {"black_sesame_seed": 2}}, {"keta_salmon": {"black_sesame_seed": 3}}, {"juniper_berry": {"black_sesame_seed": 5}}, {"cheddar_cheese": {"black_sesame_seed": 1}}, {"calytrix_tetragona_oil": {"black_sesame_seed": 1}}, {"sea_bass": {"black_sesame_seed": 3}}, {"raw_pork": {"black_sesame_seed": 1}}, {"pear": {"black_sesame_seed": 2}}, {"pike": {"black_sesame_seed": 3}}, {"peanut": {"black_sesame_seed": 1}}, {"kumquat": {"black_sesame_seed": 2}}, {"european_cranberry": {"black_sesame_seed": 7}}, {"lemongrass": {"black_sesame_seed": 1}}, {"gin": {"black_sesame_seed": 1}}, {"orange_juice": {"black_sesame_seed": 1}}, {"dried_fig": {"black_sesame_seed": 2}}, {"sassafras": {"black_sesame_seed": 1}}, {"geranium": {"black_sesame_seed": 1}}, {"java_citronella": {"black_sesame_seed": 2}}, {"bourbon_whiskey": {"black_sesame_seed": 1}}, {"peanut_butter": {"black_sesame_seed": 1}}, {"smoked_fish": {"black_sesame_seed": 3}}, {"pennyroyal": {"black_sesame_seed": 1}}, {"roasted_shrimp": {"black_sesame_seed": 1}}, {"concord_grape": {"black_sesame_seed": 3}}, {"munster_cheese": {"black_sesame_seed": 1}}, {"sage": {"black_sesame_seed": 3}}, {"cocoa": {"black_sesame_seed": 3}}, {"champagne_wine": {"black_sesame_seed": 3}}, {"chicory_root": {"black_sesame_seed": 2}}, {"chamaecyparis_formosensis_oil": {"black_sesame_seed": 1}}, {"crowberry": {"black_sesame_seed": 5}}, {"water_apple": {"black_sesame_seed": 2}}, {"mint_oil": {"black_sesame_seed": 4}}, {"wheaten_bread": {"black_sesame_seed": 1}}, {"durian": {"black_sesame_seed": 2}}, {"cloudberry": {"black_sesame_seed": 7}}, {"japanese_star_anise": {"black_sesame_seed": 1}}, {"fried_cured_pork": {"black_sesame_seed": 1}}, {"satsuma": {"black_sesame_seed": 4}}, {"winter_savory": {"black_sesame_seed": 1}}, {"cowberry": {"black_sesame_seed": 5}}, {"snap_bean": {"black_sesame_seed": 4}}, {"cabernet_sauvignon_grape": {"black_sesame_seed": 3}}, {"port_wine": {"black_sesame_seed": 3}}, {"blackberry": {"black_sesame_seed": 7}}, {"tea_tree_oil": {"black_sesame_seed": 2}}, {"shiitake": {"black_sesame_seed": 1}}, {"cumin": {"black_sesame_seed": 6}}, {"prune": {"black_sesame_seed": 2}}, {"bog_blueberry": {"black_sesame_seed": 5}}, {"celery": {"black_sesame_seed": 4}}, {"boiled_pork": {"black_sesame_seed": 1}}, {"rooibus_tea": {"black_sesame_seed": 7}}, {"eucalyptus_globulus": {"black_sesame_seed": 1}}, {"tabasco_pepper": {"black_sesame_seed": 5}}, {"fermented_shrimp": {"black_sesame_seed": 1}}, {"jackfruit": {"black_sesame_seed": 2}}, {"gruyere_cheese": {"black_sesame_seed": 3}}, {"bread": {"black_sesame_seed": 1}}, {"pineapple": {"black_sesame_seed": 1}}, {"peppermint": {"black_sesame_seed": 7}}, {"cruciferae_seed": {"black_sesame_seed": 17}}, {"israeli_orange": {"black_sesame_seed": 7}}, {"monarda_punctata_oil": {"black_sesame_seed": 1}}, {"california_orange": {"black_sesame_seed": 7}}, {"peppermint_oil": {"black_sesame_seed": 5}}, {"ceylon_tea": {"black_sesame_seed": 7}}, {"cod": {"black_sesame_seed": 3}}, {"roasted_peanut": {"black_sesame_seed": 1}}, {"fish": {"black_sesame_seed": 3}}, {"plum": {"black_sesame_seed": 3}}, {"fennel": {"black_sesame_seed": 3}}, {"toasted_oat": {"black_sesame_seed": 1}}, {"cured_pork": {"black_sesame_seed": 4}}, {"pork_liver": {"black_sesame_seed": 1}}, {"mastic_gum_fruit_oil": {"black_sesame_seed": 1}}, {"passion_fruit": {"black_sesame_seed": 7}}, {"parsley": {"black_sesame_seed": 7}}, {"soybean": {"black_sesame_seed": 4}}, {"strawberry": {"black_sesame_seed": 6}}, {"scotch_spearmint_oil": {"black_sesame_seed": 4}}, {"pork_sausage": {"black_sesame_seed": 1}}, {"hernandia_peltata_oil": {"black_sesame_seed": 1}}, {"lemongrass_oil": {"black_sesame_seed": 1}}, {"roasted_filbert": {"black_sesame_seed": 3}}, {"wild_berry": {"black_sesame_seed": 5}}, {"wine": {"black_sesame_seed": 3}}, {"raw_lean_fish": {"black_sesame_seed": 3}}, {"roasted_lamb": {"black_sesame_seed": 2}}, {"pinto_bean": {"black_sesame_seed": 4}}, {"spearmint_oil": {"black_sesame_seed": 1}}, {"chicken": {"black_sesame_seed": 1}}, {"guarana": {"black_sesame_seed": 2}}, {"ocimum_viride": {"black_sesame_seed": 1}}, {"lantana_camara_oil": {"black_sesame_seed": 1}}, {"roman_chamomile": {"black_sesame_seed": 2}}, {"mate": {"black_sesame_seed": 1}}, {"roasted_mate": {"black_sesame_seed": 1}}, {"palmarosa": {"black_sesame_seed": 1}}, {"clary_sage": {"black_sesame_seed": 1}}, {"mint": {"black_sesame_seed": 5}}, {"myrtle": {"black_sesame_seed": 4}}, {"cabernet_sauvignon_wine": {"black_sesame_seed": 3}}, {"fermented_tea": {"black_sesame_seed": 7}}, {"brown_rice": {"black_sesame_seed": 1}}, {"mastic_gum_oil": {"black_sesame_seed": 2}}, {"vanilla": {"black_sesame_seed": 3}}, {"popcorn": {"black_sesame_seed": 2}}, {"eucalyptus_macarthurii": {"black_sesame_seed": 1}}, {"shrimp": {"black_sesame_seed": 1}}, {"malagueta_pepper": {"black_sesame_seed": 5}}, {"milk": {"black_sesame_seed": 4}}, {"muscadine_grape": {"black_sesame_seed": 3}}, {"mung_bean": {"black_sesame_seed": 4}}, {"kaffir_lime": {"black_sesame_seed": 5}}, {"huckleberry": {"black_sesame_seed": 5}}, {"tangerine_juice": {"black_sesame_seed": 1}}, {"muscat_grape": {"black_sesame_seed": 3}}, {"eucalyptus_bakeries_oil": {"black_sesame_seed": 1}}, {"smoked_pork_belly": {"black_sesame_seed": 1}}, {"camembert_cheese": {"black_sesame_seed": 1}}, {"haddock": {"black_sesame_seed": 3}}, {"cymbopogon_sennaarensis": {"black_sesame_seed": 1}}, {"whitefish": {"black_sesame_seed": 3}}, {"yellow_passion_fruit": {"black_sesame_seed": 1}}, {"calytrix_tetragona": {"black_sesame_seed": 1}}, {"roasted_malt": {"black_sesame_seed": 3}}, {"thymus": {"black_sesame_seed": 6}}, {"mandarin_peel": {"black_sesame_seed": 3}}, {"loganberry": {"black_sesame_seed": 5}}, {"tangerine_peel_oil": {"black_sesame_seed": 1}}, {"mango": {"black_sesame_seed": 6}}, {"roman_chamomile_oil": {"black_sesame_seed": 1}}, {"muskmelon": {"black_sesame_seed": 3}}, {"roasted_chicory_root": {"black_sesame_seed": 2}}, {"sherry": {"black_sesame_seed": 3}}, {"fatty_fish": {"black_sesame_seed": 3}}, {"lime_juice": {"black_sesame_seed": 1}}, {"dried_black_tea": {"black_sesame_seed": 7}}, {"malay_apple": {"black_sesame_seed": 2}}, {"navy_bean": {"black_sesame_seed": 4}}, {"smoked_pork": {"black_sesame_seed": 1}}, {"mutton_liver": {"black_sesame_seed": 1}}, {"seychelles_tea": {"black_sesame_seed": 7}}, {"lime": {"black_sesame_seed": 5}}, {"raw_fish": {"black_sesame_seed": 3}}, {"papaya": {"black_sesame_seed": 5}}, {"green_tea": {"black_sesame_seed": 7}}, {"citrus_peel_oil": {"black_sesame_seed": 7}}, {"seed": {"black_sesame_seed": 17}}, {"raw_fatty_fish": {"black_sesame_seed": 3}}, {"parsnip_fruit": {"black_sesame_seed": 2}}, {"parsnip": {"black_sesame_seed": 1}}, {"blenheim_apricot": {"black_sesame_seed": 2}}, {"buchu": {"black_sesame_seed": 4}}, {"blueberry": {"black_sesame_seed": 6}}, {"sauvignon_blanc_grape": {"black_sesame_seed": 3}}, {"kiwi": {"black_sesame_seed": 2}}, {"white_wine": {"black_sesame_seed": 4}}, {"long_pepper": {"black_sesame_seed": 5}}, {"fried_pork": {"black_sesame_seed": 1}}, {"kidney_bean": {"black_sesame_seed": 4}}, {"wild_raspberry": {"black_sesame_seed": 5}}, {"licorice": {"black_sesame_seed": 3}}, {"grapefruit": {"black_sesame_seed": 4}}, {"roasted_coconut": {"black_sesame_seed": 1}}, {"buchu_oil": {"black_sesame_seed": 4}}, {"guinea_pepper": {"black_sesame_seed": 5}}, {"burley_tobacco": {"black_sesame_seed": 1}}, {"monkey_orange": {"black_sesame_seed": 4}}, {"cooked_apple": {"black_sesame_seed": 2}}, {"roasted_cocoa": {"black_sesame_seed": 3}}, {"cream_cheese": {"black_sesame_seed": 1}}, {"smoked_fatty_fish": {"black_sesame_seed": 3}}, {"oatmeal": {"black_sesame_seed": 2}}, {"ocimum_gratissimum": {"black_sesame_seed": 1}}, {"coconut": {"black_sesame_seed": 1}}, {"roasted_pecan": {"black_sesame_seed": 3}}, {"horse_mackerel": {"black_sesame_seed": 3}}, {"peach": {"black_sesame_seed": 2}}, {"dwarf_quince": {"black_sesame_seed": 2}}, {"seed_oil": {"black_sesame_seed": 3}}, {"lingonberry": {"black_sesame_seed": 2}}, {"capsicum": {"black_sesame_seed": 5}}, {"leaf": {"black_sesame_seed": 6}}, {"jasmine_tea": {"black_sesame_seed": 7}}, {"elderberry": {"black_sesame_seed": 7}}, {"cape_gooseberry": {"black_sesame_seed": 6}}, {"roasted_spanish_peanut": {"black_sesame_seed": 1}}, {"lean_fish": {"black_sesame_seed": 3}}, {"comte_cheese": {"black_sesame_seed": 1}}, {"root": {"black_sesame_seed": 1}}, {"ginger": {"black_sesame_seed": 10}}, {"cherry": {"black_sesame_seed": 2}}, {"eucalyptus_dives": {"black_sesame_seed": 1}}, {"uncured_boiled_pork": {"black_sesame_seed": 1}}, {"raw_lamb": {"black_sesame_seed": 2}}, {"salmon": {"black_sesame_seed": 3}}, {"crownberry": {"black_sesame_seed": 5}}, {"rapeseed": {"black_sesame_seed": 17}}, {"dried_parsley": {"black_sesame_seed": 7}}, {"lemon_balm": {"black_sesame_seed": 3}}, {"roasted_barley": {"black_sesame_seed": 2}}, {"chicken_liver": {"black_sesame_seed": 1}}, {"tangerine": {"black_sesame_seed": 5}}, {"cilantro": {"black_sesame_seed": 1}}, {"fenugreek": {"black_sesame_seed": 1}}, {"swiss_cheese": {"black_sesame_seed": 1}}, {"raw_chicken": {"black_sesame_seed": 1}}, {"sheep_cheese": {"black_sesame_seed": 1}}, {"celery_seed": {"black_sesame_seed": 3}}, {"french_bean": {"black_sesame_seed": 4}}, {"whiskey": {"black_sesame_seed": 3}}, {"tuber": {"black_sesame_seed": 1}}, {"grape": {"black_sesame_seed": 3}}, {"coffee": {"black_sesame_seed": 3}}, {"filbert": {"black_sesame_seed": 2}}, {"peanut_oil": {"black_sesame_seed": 1}}, {"quince": {"black_sesame_seed": 2}}, {"spanish_sage": {"black_sesame_seed": 1}}, {"lemon_peel_oil": {"black_sesame_seed": 2}}, {"smoked_herring": {"black_sesame_seed": 3}}, {"coriander": {"black_sesame_seed": 6}}, {"rice": {"black_sesame_seed": 1}}, {"cinnamon": {"black_sesame_seed": 7}}, {"roasted_pork": {"black_sesame_seed": 1}}, {"chinese_quince": {"black_sesame_seed": 3}}, {"chive": {"black_sesame_seed": 3}}, {"grapefruit_juice": {"black_sesame_seed": 4}}, {"fried_chicken": {"black_sesame_seed": 2}}, {"emmental_cheese": {"black_sesame_seed": 1}}, {"melon": {"black_sesame_seed": 3}}, {"laurel": {"black_sesame_seed": 7}}, {"nectarine": {"black_sesame_seed": 4}}, {"wort": {"black_sesame_seed": 3}}, {"rum": {"black_sesame_seed": 4}}, {"caraway_seed": {"black_sesame_seed": 1}}, {"calamus": {"black_sesame_seed": 2}}, {"lemon_peel": {"black_sesame_seed": 2}}, {"watermelon": {"black_sesame_seed": 2}}, {"capsicum_annuum": {"black_sesame_seed": 5}}, {"hop": {"black_sesame_seed": 3}}, {"uncured_pork": {"black_sesame_seed": 1}}, {"provolone_cheese": {"black_sesame_seed": 1}}, {"boiled_beef": {"black_sesame_seed": 1}}, {"mountain_papaya": {"black_sesame_seed": 2}}, {"uncured_smoked_pork": {"black_sesame_seed": 1}}, {"spearmint": {"black_sesame_seed": 1}}, {"raw_beef": {"black_sesame_seed": 1}}, {"chinese_star_anise": {"black_sesame_seed": 1}}, {"boiled_crab": {"black_sesame_seed": 1}}, {"pawpaw": {"black_sesame_seed": 1}}, {"italian_lime": {"black_sesame_seed": 5}}, {"wheat_bread": {"black_sesame_seed": 1}}, {"calabash_nutmeg": {"black_sesame_seed": 3}}, {"yeast": {"black_sesame_seed": 1}}, {"choke_cherry": {"black_sesame_seed": 2}}, {"chokeberry": {"black_sesame_seed": 5}}, {"rice_husk": {"black_sesame_seed": 1}}, {"goat_cheese": {"black_sesame_seed": 1}}, {"finocchoi_fennel_oil": {"black_sesame_seed": 1}}, {"thai_pepper": {"black_sesame_seed": 5}}, {"sauvignon_grape": {"black_sesame_seed": 3}}, {"rose_apple": {"black_sesame_seed": 3}}, {"sour_cherry": {"black_sesame_seed": 2}}, {"crisp_bread": {"black_sesame_seed": 3}}, {"pepper": {"black_sesame_seed": 5}}, {"corn_mint": {"black_sesame_seed": 5}}, {"dried_kidney_bean": {"black_sesame_seed": 4}}, {"origanum_floribundum": {"black_sesame_seed": 1}}, {"hinoki_oil": {"black_sesame_seed": 1}}, {"prickly_pear": {"black_sesame_seed": 2}}, {"porcini": {"black_sesame_seed": 1}}, {"palm": {"black_sesame_seed": 1}}, {"cumin_fruit_oil": {"black_sesame_seed": 1}}, {"raspberry": {"black_sesame_seed": 7}}, {"pimento": {"black_sesame_seed": 2}}, {"fermented_russian_black_tea": {"black_sesame_seed": 7}}, {"ceylon_citronella": {"black_sesame_seed": 1}}, {"hinoki": {"black_sesame_seed": 1}}, {"eucalyptus": {"black_sesame_seed": 6}}, {"tarragon": {"black_sesame_seed": 1}}, {"mantis_shrimp": {"black_sesame_seed": 1}}, {"citrus_peel": {"black_sesame_seed": 7}}, {"grilled_pork": {"black_sesame_seed": 1}}, {"green_bell_pepper": {"black_sesame_seed": 5}}, {"peru_balsam": {"black_sesame_seed": 2}}, {"elderberry_fruit": {"black_sesame_seed": 2}}, {"cranberry": {"black_sesame_seed": 7}}, {"red_currant": {"black_sesame_seed": 5}}, {"orange_peel": {"black_sesame_seed": 2}}, {"raw_bean": {"black_sesame_seed": 4}}, {"corn": {"black_sesame_seed": 3}}, {"galanga": {"black_sesame_seed": 1}}, {"lima_bean": {"black_sesame_seed": 4}}, {"brewed_tea": {"black_sesame_seed": 7}}, {"feta_cheese": {"black_sesame_seed": 1}}, {"butter": {"black_sesame_seed": 3}}, {"oregano_oil": {"black_sesame_seed": 1}}, {"orthodon_citraliferum": {"black_sesame_seed": 1}}, {"satureia_thymera": {"black_sesame_seed": 1}}, {"sweetfish": {"black_sesame_seed": 3}}, {"prunus": {"black_sesame_seed": 2}}, {"turmeric": {"black_sesame_seed": 4}}, {"perovski_abrotanoides_oil": {"black_sesame_seed": 1}}, {"clove_oil": {"black_sesame_seed": 2}}, {"litchi": {"black_sesame_seed": 3}}, {"kelp": {"black_sesame_seed": 1}}, {"tahiti_vanilla": {"black_sesame_seed": 2}}, {"feijoa": {"black_sesame_seed": 4}}, {"globefish": {"black_sesame_seed": 3}}, {"caraway": {"black_sesame_seed": 2}}, {"japanese_peppermint_oil": {"black_sesame_seed": 1}}, {"lovage": {"black_sesame_seed": 7}}, {"dill": {"black_sesame_seed": 9}}, {"mackerel": {"black_sesame_seed": 3}}, {"mexican_lime": {"black_sesame_seed": 5}}, {"pecan": {"black_sesame_seed": 3}}, {"mushroom": {"black_sesame_seed": 1}}, {"lovage_leaf": {"black_sesame_seed": 1}}, {"eel": {"black_sesame_seed": 3}}, {"cognac": {"black_sesame_seed": 5}}, {"fried_beef": {"black_sesame_seed": 2}}, {"red_bean": {"black_sesame_seed": 4}}, {"star_anise": {"black_sesame_seed": 1}}, {"citrus_juice": {"black_sesame_seed": 2}}, {"neroli_bigarade": {"black_sesame_seed": 1}}, {"hop_oil": {"black_sesame_seed": 3}}, {"roasted_chicken": {"black_sesame_seed": 1}}, {"blue_cheese": {"black_sesame_seed": 1}}, {"pouching_tea": {"black_sesame_seed": 7}}, {"domiati_cheese": {"black_sesame_seed": 1}}, {"callitris": {"black_sesame_seed": 1}}, {"roasted_green_tea": {"black_sesame_seed": 7}}, {"cherimoya": {"black_sesame_seed": 1}}, {"elder_flower": {"black_sesame_seed": 1}}, {"guava": {"black_sesame_seed": 6}}, {"lime_peel_oil": {"black_sesame_seed": 3}}, {"matsutake": {"black_sesame_seed": 1}}, {"olive": {"black_sesame_seed": 2}}, {"clove": {"black_sesame_seed": 5}}, {"ceylon_tea_cinnamon_leaf": {"black_sesame_seed": 1}}, {"sperm_whale_oil": {"black_sesame_seed": 1}}, {"california_orange_peel": {"black_sesame_seed": 2}}, {"rye_bread": {"black_sesame_seed": 3}}, {"citrus": {"black_sesame_seed": 4}}, {"mozzarella_cheese": {"black_sesame_seed": 1}}, {"petitgrain": {"black_sesame_seed": 1}}, {"boiled_chicken": {"black_sesame_seed": 1}}, {"roasted_turkey": {"black_sesame_seed": 1}}, {"dill_seed": {"black_sesame_seed": 17}}, {"mandarin": {"black_sesame_seed": 4}}, {"scallop": {"black_sesame_seed": 1}}, {"corn_oil": {"black_sesame_seed": 2}}, {"carrot": {"black_sesame_seed": 4}}, {"eucalyptus_globulus_oil": {"black_sesame_seed": 1}}, {"white_bread": {"black_sesame_seed": 1}}, {"java_citronella_oil": {"black_sesame_seed": 2}}, {"rosemary": {"black_sesame_seed": 5}}, {"tamarind": {"black_sesame_seed": 4}}, {"scotch_spearmint": {"black_sesame_seed": 4}}, {"rabbiteye_blueberry": {"black_sesame_seed": 5}}, {"fennel_oil": {"black_sesame_seed": 1}}, {"tilsit_cheese": {"black_sesame_seed": 1}}, {"squid": {"black_sesame_seed": 1}}, {"cardamom": {"black_sesame_seed": 7}}, {"tea": {"black_sesame_seed": 7}}, {"pilchard": {"black_sesame_seed": 3}}, {"starfruit": {"black_sesame_seed": 4}}, {"wild_strawberry": {"black_sesame_seed": 5}}, {"malt": {"black_sesame_seed": 3}}, {"tomato": {"black_sesame_seed": 3}}, {"lemon": {"black_sesame_seed": 5}}, {"loquat": {"black_sesame_seed": 3}}, {"roquefort_cheese": {"black_sesame_seed": 1}}, {"mentha_silvestris_oil": {"black_sesame_seed": 1}}, {"palm_fruit": {"black_sesame_seed": 2}}, {"mandarin_peel_oil": {"black_sesame_seed": 3}}, {"fig": {"black_sesame_seed": 2}}, {"kola_tea": {"black_sesame_seed": 7}}, {"japanese_peppermint": {"black_sesame_seed": 7}}, {"caja_fruit": {"black_sesame_seed": 2}}, {"watercress": {"black_sesame_seed": 1}}, {"hog_plum": {"black_sesame_seed": 2}}, {"buckwheat": {"black_sesame_seed": 2}}, {"red_wine": {"black_sesame_seed": 3}}, {"botrytized_wine": {"black_sesame_seed": 3}}, {"ethiopian_pepper": {"black_sesame_seed": 5}}, {"smoked_salmon": {"black_sesame_seed": 3}}, {"lamb": {"black_sesame_seed": 2}}, {"mace": {"black_sesame_seed": 6}}, {"echinacea": {"black_sesame_seed": 1}}, {"cottage_cheese": {"black_sesame_seed": 1}}]}
"#;
    let mut parser = Parser::default();
    let padded_string = PaddedString::from(data);
    let mut doc = parser.iterate(&padded_string).unwrap();

    let mut arr = doc
        .get_object()
        .unwrap()
        .at_pointer("/ingredients")
        .unwrap()
        .get_array()
        .unwrap();

    let mut arr_begin = arr.begin().unwrap();
    let arr_end = arr.end().unwrap();

    let mut index = 0;
    while arr_begin.not_equal(&arr_end) {
        dbg!(index);
        let mut value = arr_begin.next();

        let mut object = value.get_object().unwrap();

        let mut object_begin = object.begin().unwrap();
        let object_end = object.end().unwrap();

        while object_begin.not_equal(&object_end) {
            let mut field = object_begin.next();

            println!("{}-{}", index, field.unescaped_key().unwrap());
        }

        index += 1;
    }
}
