extern crate photon_rs;

use photon_rs::native::*;

pub async fn build_award_image(user_img_url: &String) -> Result<String, ()> {
    let r_client = reqwest::Client::builder().build().unwrap();
    let profile_picture = r_client
        .get(format!("{}?size=128", user_img_url))
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();


    let mut pfp_photon = open_image_from_bytes(profile_picture.as_ref()).expect("The profile-pic should be open");
    let mask_photon = open_image("img/blackcomposite.png").expect("mask.png should be open");
    photon_rs::multiple::watermark(&mut pfp_photon, &mask_photon, 0, 0);
    save_image(pfp_photon, "pfp_new.png");
    Ok("pfp_new.png".to_string())
}

