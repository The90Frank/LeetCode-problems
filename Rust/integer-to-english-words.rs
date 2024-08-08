impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        
        let mut ret_string : String =  "".to_owned();
        
        let map_unit_0_19 = vec![
            "",
            "One ",	        
            "Two ",	    
            "Three ",	
            "Four ",
            "Five ",
            "Six ",    
            "Seven ",	
            "Eight ",
            "Nine ",
            "Ten ",    
            "Eleven ",	
            "Twelve ",
            "Thirteen ",	
            "Fourteen ",
            "Fifteen ",	    
            "Sixteen ",	    
            "Seventeen ",	
            "Eighteen ",	
            "Nineteen "
        ];

        let map_tenth_0_90 = vec![
            "",
            "Ten ",
            "Twenty ",
            "Thirty ",
            "Forty ",	
            "Fifty ",	
            "Sixty ",
            "Seventy ",
            "Eighty ",
            "Ninety "
        ];

        let chunk_num = vec![
            (num / 1000000000) % 1000,
            (num / 1000000) % 1000,
            (num / 1000) % 1000,
            (num / 1) % 1000,
        ];

        let string_1000 = vec![
            "Billion ",
            "Million ", 
            "Thousand ",
            ""
        ];

        for i in 0..chunk_num.len() {
            let cur_num = chunk_num[i];
            if cur_num != 0 {
                if cur_num >= 100 {
                   ret_string.push_str(map_unit_0_19[ (cur_num/100) as usize ]);
                   ret_string.push_str("Hundred ");
                }

                if cur_num % 100 < 20 {
                    ret_string.push_str(map_unit_0_19[ (cur_num%100) as usize ]);
                } else {
                    ret_string.push_str(map_tenth_0_90[ ((cur_num%100) / 10) as usize ]);
                    ret_string.push_str(map_unit_0_19[ (cur_num%10) as usize ]);
                }

                ret_string.push_str(string_1000[i]);
            }
        }

        return ret_string.trim().to_string();
    }
}
