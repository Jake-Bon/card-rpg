use serde::Deserialize;

pub struct NPC {

    npcName: String,
    deckID: i32,
    dialog: []String,

}



fn initialize() -> rdr {
    // Build the CSV reader and iterate over each record.
    let rdr: vec<NPC> = serde_json::from_str(npc_loader.json);
    Ok(())
}
