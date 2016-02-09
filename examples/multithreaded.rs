extern crate mecab;

use mecab::Model;

fn main() {
    let input = "太郎は次郎が持っている本を花子に渡した。";

    // create model object
    let model = Model::new("");

    // create tagger based on the model
    let tagger = model.create_tagger();

    // create lattice object per thread
    let mut lattice = model.create_lattice();

    // get tagged result as string
    lattice.set_sentence(input);

    // parse lattice
    tagger.parse(&lattice);
    println!("{}", lattice.to_string());

    // iterate over node objects
    for node in lattice.bos_node().iter_next() {
        match node.stat as i32 {
            mecab::MECAB_BOS_NODE => {
                print!("{} BOS ", node.id);
            }
            mecab::MECAB_EOS_NODE => {
                print!("{} EOS ", node.id);
            }
            _ => {
                print!("{} {} ", node.id, &(node.surface)[..(node.length as usize)]);
            }
        }

        println!("{} {} {} {} {} {} {} {} {} {} {} {} {}",
                 node.feature,
                 input.len() as isize - node.surface.len() as isize,
                 input.len() as isize - node.surface.len() as isize + node.length as isize,
                 node.rcattr,
                 node.lcattr,
                 node.posid,
                 node.char_type,
                 node.stat,
                 node.isbest,
                 node.alpha,
                 node.beta,
                 node.prob,
                 node.cost);
    }

    // iterate over begin and end nodes
    let len = lattice.size();
    for i in 0..len + 1 {
        let b = lattice.begin_nodes(i);
        let e = lattice.end_nodes(i);

        if let Some(nodes) = b {
            for node in nodes.iter_bnext() {
                println!("B[{}] {}\t{}", i, node.surface, node.feature);
            }
        }

        if let Some(nodes) = e {
            for node in nodes.iter_enext() {
                println!("E[{}] {}\t{}", i, node.surface, node.feature);
            }
        }
    }

    // get N best results
    lattice.set_request_type(mecab::MECAB_NBEST);
    lattice.set_sentence(input);
    tagger.parse(&lattice);

    for i in 0..10 {
        println!("NBEST: {}", i);
        println!("{}", lattice.to_string());

        if !lattice.next() {
            break;
        }
    }

    // marginal probabilities
    lattice.remove_request_type(mecab::MECAB_NBEST);
    lattice.set_request_type(mecab::MECAB_MARGINAL_PROB);
    lattice.set_sentence(input);
    tagger.parse(&lattice);

    println!("{}", lattice.theta());

    for node in lattice.bos_node().iter_next() {
        println!("{}\t{}\t{}",
                 &(node.surface)[..(node.length as usize)],
                 node.feature,
                 node.prob);
    }

    // dictionary info
    for dict in model.dictionary_info().iter() {
        println!("\nfilename: {}", dict.filename);
        println!("charset: {}", dict.charset);
        println!("size: {}", dict.size);
        println!("type: {}", dict.dict_type);
        println!("lsize: {}", dict.lsize);
        println!("rsize: {}", dict.rsize);
        println!("version: {}", dict.version);
    }
}
