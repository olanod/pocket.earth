use capnp::message::Builder;
use osmpbf::{Element, ElementReader};
use s2::{cellid::CellID, latlng::LatLng, s1::Deg};
use std::io::stdin;

#[allow(dead_code)]
mod earth_capnp;
use earth_capnp::planet;

fn main() -> Result<(), failure::Error> {
    let reader = ElementReader::new(stdin());
    let mut msg = Builder::new_default();
    let _world = msg.init_root::<planet::Builder>();

    let cells = reader.par_map_reduce(
        |element| {
            vec![match element {
                Element::DenseNode(n) => cell_from_ll(n.lat(), n.lon()),
                Element::Node(n) => cell_from_ll(n.lat(), n.lon()),
                _ => CellID(0),
            }]
        },
        || vec![],
        |mut a, b| {
            if !b.is_empty() && b[0].is_valid() {
                a.extend(&b);
            }
            a
        },
    )?;

    println!("Number of cells/nodes: {}", cells.len());
    Ok(())
}

fn cell_from_ll(lat: f64, lng: f64) -> CellID {
    LatLng::new(Deg(lat).into(), Deg(lng).into()).into()
}
