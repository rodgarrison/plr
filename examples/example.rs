use plr::regression::GreedyPLR;                                                                                     

fn main() {
  // first, generate some data points...                                                                              
  let mut data = Vec::new();                                                                                          
                                                                                                                     
  data.push((0.0, 65.41918966083428859105971153));
  data.push((1.0, 65.41918966083428867187647548));
  data.push((2.0, 65.41918966083428906114600250));
  data.push((3.0, 65.41918966083428935491998504));
  data.push((4.0, 65.41918966083428939639400668));
  data.push((5.0, 65.41918966083428939908098657));
  data.push((6.0, 65.41918966083429076430762160));
                                                                                                                     
  let mut plr = GreedyPLR::new(4.0);
                                                                                                                     
  let mut segments = Vec::new();                                                                                      
                                                                                                                     
  for (x, y) in data {                                                                                                
    // when `process` returns a segment, we should add it to our list                                               
    if let Some(segment) = plr.process(x, y) {                                                                      
      segments.push(segment);                                                                                     
    }                                                                                                               
  }                                                                                                                   
                                                                                                                     
  // because we have a finite amount of data, we flush the buffer and get the potential                               
  // last segment.                                                                                                    
  if let Some(segment) = plr.finish() {                                                                               
    segments.push(segment);                                                                                         
  } 
}
