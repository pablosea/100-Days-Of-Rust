fn main() {
    get_football_fields(9762.0);
}

fn get_football_fields(area: f64){

    let total_area_km: f64 = area * 1000000.0;
    let euro_field: f64 = 105.0 * 68.0;
    let american_field: f64 = 110.0 * 49.0;

    let total_football_pitches: f64 = total_area_km / euro_field;
    let total_football_fields: f64 = total_area_km / american_field;
    
    println!("The area of deforestation is {} european football pitches, or {} American football fields.", 
                total_football_pitches, 
                total_football_fields
            );
}
