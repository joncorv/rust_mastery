// action log anomolies
// our input data is a vec of tuples with: (agent, status, ticket_num)

// we need to find all the anomolies, aka anythng outside this:
// ticket is opened and closed once in that order
// ticket is opened and closed by same agent
// the agent didn't do any other actions for a different ticket between opening and closing the
// current ticket

// input data is Vec<(String, String, u32)>
// gonna need a hashmaps
// ticket_status_hash = (ticket_num, status)
// agent_latest_ticket =  (agent, ticket_num)

// iterate over data
    // validate ticket_status
        // match the status.
            // if status is open, 
                // make sure it doesn't exist in HashMap. 
                // if it does exist, add it to anomolies
            //if status is closed
                // make sure it exists in HashMap
                // if it doesn't exist in hashmap - add to anomolie
                    // if status is anything but open, add to anomolies
    // validate agent 
        //match status
            // if status is open
                // make sure agent is not in agent_latest_ticket
                    // or else add to anomolie
