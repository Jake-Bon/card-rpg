use serde::Deserialize;

//name is the name of the enemy
//deck id is what the deck is made up of (we had a txt file somewhere containing the cards)
//dialog is the dialog from the enemy
pub struct jsonNPC
{
    inlist: vec<u32>,
    npcs: vec<npcData>
    {
        npcName: String,
        deckID: i32,
        predialog: vec<String>,
        postdialog: vec<String>,
        sprites: vec<String>,
    },
}

impl<'a> jsonNPC<'a>
{
    pub fn npcInitialize() -> rdr
    {
        // Build the CSV reader and iterate over each record.
        let npcs: vec<npcData> = serde_json::from_str(npc_loader.json);
        let inlist = npcs.len();
        Ok(())
    }
}
