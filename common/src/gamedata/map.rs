
use std::ops::{Index, IndexMut};
use array2d::*;
use objholder::*;
use basic::{MAX_ITEM_FOR_DRAW, MAX_TILE_IMG_OVERLAP};
use gamedata::item::ItemList;
use gamedata::chara::CharaId;
use gamedata::site::SiteId;
use gamedata::region::RegionId;

pub use piece_pattern::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub w: u32,
    pub h: u32,
    pub tile: Array2d<TileInfo>,
    pub observed_tile: Array2d<ObservedTileInfo>,
    pub player_pos: Vec2d,
    pub entrance: Vec2d,
    /// Characters on this map
    charaid: Vec<CharaId>,
    /// This is drawed outer this map
    /// If this is None, nearest tile's infomation will be used
    pub outside_tile: Option<OutsideTileInfo>,
    pub boundary: MapBoundary,
}

/// Represents overlapped tile images
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct OverlappedTile(pub [TileIdxPP; MAX_TILE_IMG_OVERLAP]);

impl Default for OverlappedTile {
    fn default() -> OverlappedTile {
        let tile = TileIdxPP {
            idx: TileIdx::default(),
            piece_pattern: PiecePattern::EMPTY,
        };
        let mut overlapped_tile = [tile; MAX_TILE_IMG_OVERLAP];
        overlapped_tile[0].piece_pattern = PiecePattern::SURROUNDED;
        OverlappedTile(overlapped_tile)
    }
}

impl From<TileIdx> for OverlappedTile {
    fn from(tile_idx: TileIdx) -> OverlappedTile {
        let mut overlapped_tile = OverlappedTile::default();
        overlapped_tile[0].idx = tile_idx;
        overlapped_tile
    }
}

impl Index<usize> for OverlappedTile {
    type Output = TileIdxPP;
    fn index(&self, index: usize) -> &TileIdxPP {
        &self.0[index]
    }
}

impl IndexMut<usize> for OverlappedTile {
    fn index_mut(&mut self, index: usize) -> &mut TileIdxPP {
        &mut self.0[index]
    }
}

impl OverlappedTile {
    pub fn len(&self) -> usize {
        for i in 0..MAX_TILE_IMG_OVERLAP {
            if self[i].is_empty() {
                return i;
            }
        }
        MAX_TILE_IMG_OVERLAP
    }

    pub fn main_tile(&self) -> TileIdx {
        assert!(self.len() > 0);
        self[self.len() - 1].idx
    }
}

/// This represents special objects on a tile. For example, stairs, doors, traps.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SpecialTileKind {
    None,
    Stairs {
        /// Stairs has the destination floor number
        dest_floor: u32,
        kind: StairsKind,
    },
    /// Site symbol on region map
    SiteSymbol {
        kind: SiteSymbolKind,
    }
}

impl SpecialTileKind {
    pub fn is_none(&self) -> bool {
        match *self {
            SpecialTileKind::None => true,
            _ => false,
        }
    }
}

impl Default for SpecialTileKind {
    fn default() -> SpecialTileKind {
        SpecialTileKind::None
            
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum StairsKind {
    UpStairs, DownStairs,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SiteSymbolKind {
    Cave, Tower, Town, Village,
}

impl SpecialTileKind {
    /// Convert to id of SpecialTileObject
    pub fn obj_id(&self) -> Option<&'static str> {
        Some(match *self {
            SpecialTileKind::None => { return None; },
            SpecialTileKind::Stairs { kind, .. } => {
                match kind {
                    StairsKind::DownStairs => "!downstairs",
                    StairsKind::UpStairs => "!upstairs",
                }
            }
            SpecialTileKind::SiteSymbol { kind } => {
                match kind {
                    SiteSymbolKind::Cave =>    "!rm.cave",
                    SiteSymbolKind::Tower =>   "!rm.tower",
                    SiteSymbolKind::Town =>    "!rm.town",
                    SiteSymbolKind::Village => "!rm.village",
                }
            }
        })
    }
}

/// If stairs or boundaries have this value, they are connected to region map
pub const FLOOR_OUTSIDE: u32 = 0xFFFFFFFF;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileInfo {
    /// Base tile
    pub tile: OverlappedTile,
    /// If wall is presented, the tile is no walkable
    pub wall: WallIdxPP, 
    /// Decoration for this tile
    pub deco: Option<DecoIdx>,
    /// Items on this tile
    pub item_list: Option<ItemList>,
    pub chara: Option<CharaId>,
    pub special: SpecialTileKind,
}

/// The data for map drawing
/// These data will be updated every player turn based on player's view
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct ObservedTileInfo {
    pub tile: Option<OverlappedTile>,
    pub wall: WallIdxPP,
    pub deco: Option<DecoIdx>,
    pub n_item: usize,
    pub items: [ItemIdx; MAX_ITEM_FOR_DRAW],
    pub special: SpecialTileKind,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OutsideTileInfo {
    pub tile: TileIdx,
    pub wall: Option<WallIdx>, 
    pub deco: Option<DecoIdx>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MapBoundary {
    pub n: BoundaryBehavior,
    pub s: BoundaryBehavior,
    pub e: BoundaryBehavior,
    pub w: BoundaryBehavior,
}

/// Reperesents the floor that boundary connect to
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum BoundaryBehavior {
    None, Floor(u32), RegionMap, MapId(MapId, u32),
}

impl Default for BoundaryBehavior {
    fn default() -> BoundaryBehavior {
        BoundaryBehavior::None
    }
}
         
impl Default for TileInfo {
    fn default() -> TileInfo {
        TileInfo {
            tile: OverlappedTile::default(),
            wall: WallIdxPP::default(),
            deco: None,
            item_list: None,
            chara: None,
            special: SpecialTileKind::None,
        }
    }
}

impl Map {
    pub fn new(w: u32, h: u32) -> Map {
        Map {
            w: w, h: h,
            tile: Array2d::new(w, h, TileInfo::default()),
            observed_tile: Array2d::new(w, h, ObservedTileInfo::default()),
            player_pos: Vec2d::new(0, 0), entrance: Vec2d::new(0, 0),
            charaid: Vec::new(),
            outside_tile: None,
            boundary: MapBoundary::default(),
        }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn size(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    pub fn get_chara<T: Into<Vec2d>>(&self, pos: T) -> Option<CharaId> {
        let pos = pos.into();
        if !self.is_inside(pos) { return None; }
        self.tile[pos].chara.clone()
    }

    pub fn iter_charaid(&self) -> ::std::slice::Iter<CharaId> {
        self.charaid.iter()
    }

    pub fn is_movable(&self, pos: Vec2d) -> bool {
        if !self.is_inside(pos) {
            return false;
        }

        self.tile[pos].wall.is_empty()
    }

    /// Return given pos is inside map or not
    #[inline]
    pub fn is_inside(&self, pos: Vec2d) -> bool {
        if pos.0 >= 0 && pos.1 >= 0 && (pos.0 as u32) < self.w && (pos.1 as u32) < self.h {
            true
        }else{
            false
        }
    }

    /// Add one character on this map.
    pub(crate) fn add_chara(&mut self, pos: Vec2d, id: CharaId) {
        self.charaid.push(id);
        self.tile[pos].chara = Some(id);
    }

    /// Get character position
    pub fn chara_pos(&self, cid: CharaId) -> Option<Vec2d> {
        for p in self.tile.iter_idx() {
            if self.tile[p].chara.as_ref() == Some(&cid) {
                return Some(p);
            }
        }
        None
    }

    pub fn move_chara(&mut self, cid: CharaId, dir: Direction) -> bool {
        use std::mem::replace;
        if let Some(p) = self.chara_pos(cid) {
            let new_p = p + dir.as_vec();
            if self.is_movable(p + dir.as_vec()) { // Swap charas of the two tiles
                let temp0 = replace(&mut self.tile[p].chara,     None);
                let temp1 = replace(&mut self.tile[new_p].chara, None);
                replace(&mut self.tile[p].chara,     temp1);
                replace(&mut self.tile[new_p].chara, temp0);
                true
            }else{
                false
            }
        }else{
            false
        }
    }

    /// Locate a character at given position.
    /// If the new position is not empty, this function will fail and return false
    pub fn locate_chara(&mut self, cid: CharaId, pos: Vec2d) -> bool {
        if self.tile[pos].chara.is_some() { return false; }
            
        if let Some(old_pos) = self.chara_pos(cid) {
            self.tile[old_pos].chara = None;
        }
        self.tile[pos].chara = Some(cid);
        true
    }

    pub(crate) fn search_empty_onmap_charaid_n(&self) -> u32 {
        'i_loop:
        for i in 0.. {
            for cid in self.charaid.iter() {
                if let CharaId::OnMap { n, .. } = *cid {
                    if i == n { continue 'i_loop; }
                }
            }
            return i;
        }
        panic!()
    }

    pub(crate) fn remove_chara(&mut self, cid: CharaId) {
        let pos = self.chara_pos(cid).unwrap();
        self.tile[pos].chara = None;

        if let CharaId::OnMap { .. } = cid {
            let i = self.charaid.iter().enumerate().find(|&(_, cid_o)| *cid_o == cid).unwrap().0;
            let removed_cid = self.charaid.swap_remove(i);
            assert!(removed_cid == cid);
        }
    }

    /// Get tile index with extrapolation
    /// If pos is outside map and self.outside_tile has value, returns it.
    /// If pos is outside map and self.outside_tile is None, returns the nearest tile.
    pub fn get_tile_extrapolated(&self, pos: Vec2d) -> OverlappedTile {
        if self.is_inside(pos) {
            return self.tile[pos].tile;
        }
        if let Some(outside_tile) = self.outside_tile {
            outside_tile.tile.into()
        } else {
            self.tile[self.nearest_existent_tile(pos)].tile
        }
    }

    pub fn get_wall_extrapolated(&self, pos: Vec2d) -> WallIdxPP {
        if self.is_inside(pos) {
            return self.tile[pos].wall;
        }
        if let Some(outside_tile) = self.outside_tile {
            if let Some(idx) = outside_tile.wall {
                WallIdxPP {
                    idx: idx,
                    piece_pattern: PiecePattern::SURROUNDED,
                }
            } else {
                WallIdxPP::default()
            }
        } else {
            self.tile[self.nearest_existent_tile(pos)].wall
        }
    }

    pub fn get_deco_extrapolated(&self, pos: Vec2d) -> Option<DecoIdx> {
        if self.is_inside(pos) {
            return self.tile[pos].deco;
        }
        if let Some(outside_tile) = self.outside_tile {
            outside_tile.deco
        } else {
            self.tile[self.nearest_existent_tile(pos)].deco
        }
    }

    /// Calculate the position of nearest and exsitent tile
    pub fn nearest_existent_tile(&self, pos: Vec2d) -> Vec2d {
        if self.is_inside(pos) {
            return pos;
        }
        let (w, h) = (self.w as i32, self.h as i32);
        let x = if pos.0 < 0 { 0 } else if pos.0 >= w { w - 1 } else { pos.0 };
        let y = if pos.1 < 0 { 0 } else if pos.1 >= h { h - 1 } else { pos.1 };
        Vec2d::new(x, y)
    }

    /// Search stairs that is connected to given floor
    pub fn search_stairs(&self, floor: u32) -> Option<Vec2d> {
        for (p, tile) in self.tile.iter_with_idx() {
            if let SpecialTileKind::Stairs { dest_floor, .. } = tile.special {
                if dest_floor == floor {
                    return Some(p);
                }
            }
        }
        None
    }

    /// Returns boundart when player move from 'pos' to 'dir' direction
    /// If its destination tile is inside map, return None
    pub fn get_boundary_by_tile_and_dir(&self, pos: Vec2d, dir: Direction) -> Option<BoundaryBehavior> {
        let dest_pos = pos + dir.as_vec();
        if dest_pos.0 < 0 {
            Some(self.boundary.n)
        } else if dest_pos.0 >= self.h as i32 {
            Some(self.boundary.s)
        } else if dest_pos.1 < 0 {
            Some(self.boundary.w)
        } else if dest_pos.1 >= self.w as i32 {
            Some(self.boundary.e)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum MapId {
    SiteMap { sid: SiteId, floor: u32 },
    RegionMap { rid: RegionId },
}

impl MapId {
    pub fn site_first_floor(sid: SiteId) -> MapId {
        MapId::SiteMap { sid, floor: 0 }
    }

    pub fn set_floor(self, floor: u32) -> MapId {
        match self {
            MapId::SiteMap { sid, .. } => {
                MapId::SiteMap { sid, floor }
            }
            _ => panic!("Invalid operation on MapId::RegionId")
        }
    }

    #[inline]
    pub fn rid(self) -> RegionId {
        match self {
            MapId::SiteMap { sid, .. } => sid.rid,
            MapId::RegionMap { rid } => rid,
        }
    }

    #[inline]
    pub fn sid(self) -> SiteId {
        match self {
            MapId::SiteMap { sid, .. } => sid,
            _ => panic!("Invalid operation on MapId::RegionId")
        }
    }

    #[inline]
    /// Get floor number of this map
    /// If the map is region map, returns FLOOR_OUTSIDE
    pub fn floor(self) -> u32 {
        match self {
            MapId::SiteMap { floor, .. } => floor,
            MapId::RegionMap { .. } => FLOOR_OUTSIDE,
        }
    }

    pub fn is_region_map(self) -> bool {
        match self {
            MapId::RegionMap { .. } => true,
            _ => false,
        }
    }
}

impl Default for MapId {
    fn default() -> MapId {
        MapId::SiteMap { sid: SiteId::default(), floor: 0 }
    }
}

impl From<RegionId> for MapId {
    fn from(rid: RegionId) -> MapId {
        MapId::RegionMap { rid }
    }
}

