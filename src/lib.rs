pub fn different_cases(a: usize,c: &[&str]){
    let mut count = 0;// this is here to make sure we dont loop till the end, also helps us iterate through new_vec :)
    let mut new_vec = {
        let mut nv =  Vec::with_capacity(a + 1);
        nv.push(0_usize);
        nv
    }; // this vector is contains the positions of the beginning,
    // where breaks are found and the end. if you're dum it contains 0 because beginning 🤦‍♀️
    for i in 0..c.len()-1{
        if c[i] == ""{
            new_vec.push(i+1);// i added 1 so we can skip over the breaks when we are slicing
        }
    }// finding breaks+1 and pushing them in
    new_vec.push(c.len()); // pushing the end
    loop{
        sort_and_print(&c[new_vec[count]..new_vec[count+1]]);// doing the main work
        count +=1; // adding to count 
        if count == a{
            break;
        }// this is here so continue looping. if that makes sense.
    }
}

fn sort_and_print(b: &[&str]){
    let mut arr_count = 0;
    let members = b[arr_count].parse::<usize>().unwrap();
    let mut new_vec = Vec::with_capacity(b.len()-1);
    for _ in 0..members{
        arr_count+=1;
        new_vec.push(b[arr_count].parse::<usize>().unwrap());
    }
    new_vec.sort();
    let (least_val, start_from) = {
        if new_vec.len() % 2 == 0{
            (None, 0)
        }else{
            (Some(new_vec[0]),1)
        }
    };

    let mut vec1 = Vec::with_capacity(b.len()/2 + 1);
    let mut vec2 = Vec::with_capacity(b.len()/2);
    let mut c = true; // helps separate the elements into various vecs
    for i in start_from..new_vec.len(){
        if c{
            vec1.push(new_vec[i]);
        }else {
            vec2.push(new_vec[i]);
        }
        c = !c;
    }

    let (mut difference, mut vec1sum, mut vec2sum) = diff_sum_sum(&vec1, &vec2);


    let checker  = vec2.len();

    if vec2[checker-1] > vec1sum{
        let mut least_count = 0_usize;
        let mut great_count = checker.wrapping_sub(2);

        loop {
            if great_count > vec2.len(){
                break;
            }
            if vec1[least_count] < vec2[great_count]{
                let small_num = vec1[least_count];
                vec1[least_count] = vec2[great_count];
                vec2[great_count] = small_num;
                great_count = great_count.wrapping_sub(1);
                least_count += 1;
            }else{
                break;
            }
        }
        
    }else{
        let mut count = 0;
        loop {

            if (vec2sum - vec2[count] + vec1[count]).wrapping_sub(vec1sum - vec1[count] + vec2[count]) < difference{
                let smaller = vec1[count];
                vec1[count] = vec2[count];
                vec2[count] =  smaller;
                let reassign =  diff_sum_sum(&vec1, &vec2);
                difference = reassign.0;
                vec1sum = reassign.1;
                vec2sum = reassign.2;
                count += 1;
            }else{
                break;
            }
        }
    }
    match least_val{
        Some(e)=>{
            vec1.push(e);
        },
        None=>{}
    }

    let mut sorted_sum = vec![vec1.iter().sum::<usize>(),vec2.iter().sum::<usize>()];
    sorted_sum.sort();
    println!("{} {}",sorted_sum[0],sorted_sum[1]);
}

fn diff_sum_sum(vec1: &Vec<usize>,  vec2: &Vec<usize>) -> (usize, usize, usize){
    let vec1_sum = vec1.iter().sum::<usize>();
    let vec2_sum = vec2.iter().sum::<usize>();
    (
        vec2_sum - vec1_sum,
        vec1_sum,
        vec2_sum

    )
}