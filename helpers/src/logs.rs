use rusoto::Region;
use rusoto::logs::{
    CloudWatchLogsClient,
    CloudWatchLogs,
    FilterLogEventsRequest,
//     FilterLogEventsResponse,
//     FilterLogEventsError,
    FilteredLogEvent
};

struct FilterLogEventsRequestIter {
    client: CloudWatchLogsClient,
    req: FilterLogEventsRequest,
    // Keep track of which events we have yielded already so we don't double up
    yielded_events: Vec<String>,
    // All the events we have downloaded but not tried to yield yet
    buffered_events: Vec<FilteredLogEvent>,
}

// TODO: Implement the Stream trait
// impl Stream for FilterLogEventsRequestIter {
//     type Item;
//     type Error;
//     fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>;
// }

// Iterator helper for paginating over the `filter_log_events` API call
impl Iterator for FilterLogEventsRequestIter {
    type Item = FilteredLogEvent;

    fn next(&mut self) -> Option<FilteredLogEvent> {

        // Return the next event in the buffer if there is one
        if self.buffered_events.len() > 0 {
            let ev = self.buffered_events.pop();
            return try_return(ev, &mut self.yielded_events);
        }

        // Request the next batch of events when out of buffered ones
        match self.client.filter_log_events(&self.req).sync() {
            Ok(output) => {

                // Add downloaded events to the buffer
                if output.events.is_some() {
                    let new_events = &mut (output.events.unwrap());
                    new_events.reverse();
                    self.buffered_events.append(new_events);
                }

                // Update the next_token for the next API call
                if output.next_token.is_some() {
                    self.req.next_token = output.next_token;
                }

                // Return the first newly downloaded event if there is one
                let ev = self.buffered_events.pop();
                try_return(ev, &mut self.yielded_events)
            },
            Err(error) => {
                println!("AWS Cloudwatch Error: {:?}", error);
                None
            }
        }
    }
}

// Helper fn to only return log events that have an `event_id` and haven't been yielded already
fn try_return(ev: Option<FilteredLogEvent>, yielded_events: &mut Vec<String>) -> Option<FilteredLogEvent> {

    // TODO: Shouldn't need to clone here
    ev.clone()
        .and_then(| event | { event.event_id })
        .and_then(| event_id | {
            if !yielded_events.contains(&event_id) {
                yielded_events.push(event_id);
                return ev;
            } else {
                None
            }
        })
}

