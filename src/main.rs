use std::env;
use std::mem::size_of;


type udata_t = u16;
type  data_t = i16;


const MAX_NUM_POINTS: usize = 5120;
const SIMD_LANE_BIT_WIDTH: usize = size_of::<udata_t>() * 8;
const MAX_NUM_POINT_GROUPS: usize = MAX_NUM_POINTS / SIMD_LANE_BIT_WIDTH;



struct Cloud {
	x_arr: [ data_t; MAX_NUM_POINTS],
	y_arr: [ data_t; MAX_NUM_POINTS],
	z_arr: [udata_t; MAX_NUM_POINTS],
	
	num_neighbors_arr: [udata_t; MAX_NUM_POINTS],
	cluster_idx_arr:   [udata_t; MAX_NUM_POINTS],
	type_arr:          [udata_t; MAX_NUM_POINTS],
	
	num_points: usize,
	width:      usize,
}

fn init_cloud () {
}

fn get_graph () {
}

fn get_clusters () {
}

fn draw_clusters () {
}

fn main() {
    //~ let args: Vec<String> = env::args().collect();
    //~ println!("{:?}", args);
    
    //~ let a = &args[1];
    //~ println!("arg 0 is {}", a);
    
    let neighbors_limit  = 1;
    let distance_limit   = 1.0;
    let num_input_points = 128;
    
    let mut cloud = Cloud {
		x_arr: [0; MAX_NUM_POINTS],
		y_arr: [0; MAX_NUM_POINTS],
		z_arr: [0; MAX_NUM_POINTS],
		num_neighbors_arr: [0; MAX_NUM_POINTS],
		cluster_idx_arr:   [0; MAX_NUM_POINTS],
		type_arr:          [0; MAX_NUM_POINTS],
		num_points: 0,
		width: 0,
	};
	
	init_cloud ();
	
	let num_points = cloud.num_points;
	let num_groups = num_points / SIMD_LANE_BIT_WIDTH;
	
	//let adj_mtx_size = num_points * MAX_NUM_POINT_GROUPS; 
	const adj_mtx_size: usize = MAX_NUM_POINTS * MAX_NUM_POINT_GROUPS; 
	
	let mut adj_mtx: [udata_t; adj_mtx_size] = [0; adj_mtx_size];
	
	get_graph ();
	
	get_clusters ();
	
	draw_clusters ();
    
}
