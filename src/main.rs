use std::{path::Path, collections::{HashMap, btree_map::Iter}, vec};

use ml_data::Node;

mod ml_data;

fn main() 
{
    let myiter = ml_data::read_ml_json(Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json"))
    .element_statistics.nodes.into_iter().find(|node|
    {
        if let Some(XX) = node.a.get("XX")
        { if XX == "true" 
            {
                return true;
            }
        }
        return false;
    } ).unwrap();   
    let cuiter = ml_data::read_ml_json(Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json"))
    .element_statistics.nodes;
    let nodemap = mnode_to_hashm(cuiter);
    let vecnum = corrlacion(myiter.a, &nodemap);
    let tup = maxi(vecnum);
    print!("{:?} {:?}", tup.0, tup.1);
}

fn maxi(list: Vec<f64>) -> (f64,f64)
{
    let mut maxi = 0.0;
    let mut num = 0.0;
    for k in list 
    {
        if(k > maxi) 
        {
            maxi = k;
            num = 1.0;
        }
        else if (k == maxi) {
            num = num + 1.0;
        }
    }
    (maxi,num)
}

fn corrlacion(nodo: HashMap<String, String>, vnodos: &Vec<HashMap<String, String>>) -> Vec<f64>
{
    let tam = (nodo.len() - 5) as f64;
    vnodos.iter().map(|g|
    {
        let mut sum = 0.0;
        for (k,v) in nodo.iter() 
        {
            let mut aux = 0;
            if(k == "XX") {
                aux = 1;
            }
            if(k == "LT"){
                aux = 1;
            }
            if(k == "TP") {
                aux = 1;
            }
            if(k == "WH"){
                aux = 1;
            }
            if(k == "HT") {
                aux = 1;
            }
            if(aux == 0) {
                sum += g.iter().filter(|(gk,gv)| *gk == k && *gv == v).count() as f64;
            }
        }
        sum/tam
    }).collect()
}

fn mnode_to_hashm(nodes: Vec<Node>) -> Vec<HashMap<String,String>> 
{
    nodes.into_iter().map(|n|n.a.clone()).collect()
}
