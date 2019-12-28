//! ## Client
//!
//! `client` is the module which takes care of managin the OctopipesClient struct and
//! then all the functions useful for the user to interface with an Octopipes Server

//
//   RustyPipes
//   Developed by Christian Visintin
//
// MIT License
// Copyright (c) 2019-2020 Christian Visintin
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use super::OctopipesCapError;
use super::OctopipesClient;
use super::OctopipesError;
use super::OctopipesMessage;
use super::OctopipesOptions;
use super::OctopipesProtocolVersion;
use super::OctopipesState;

use std::sync::{Arc, Mutex};
use std::thread;

impl OctopipesClient {
    /// ### OctopipesClient Constructor
    ///
    /// `new` is constructor for OctopipesClient
    pub fn new(
        client_id: String,
        cap_pipe: String,
        version: OctopipesProtocolVersion,
    ) -> OctopipesClient {
        let client = OctopipesClient {
            id: client_id,
            version: version,
            cap_pipe: cap_pipe,
            tx_pipe: None,
            rx_pipe: None,
            state: OctopipesState::Initialized,
            client_rc: None,
            client_loop: None,
            on_received_fn: None,
            on_sent_fn: None,
            on_subscribed_fn: None,
            on_unsubscribed_fn: None,
        };
        client.client_rc = Some(Arc::new(Mutex::new(client)));
        client
    }

    //Thread operations

    /// ###  loop_start
    ///
    /// `loop_start` starts the client loop thread which checks if new messages are available
    pub fn loop_start(&mut self) -> Result<(), OctopipesError> {
        match self.state {
            OctopipesState::Subscribed => {
                //Create threaded client
                if self.client_rc.is_none() {
                    return Err(OctopipesError::ThreadError)
                }
                let this = Arc::clone(&self.client_rc.unwrap());
                self.client_loop = Some(thread::spawn(move || {
                    //TODO: implement thread
                    loop {
                        let client = this.lock().unwrap();
                        if client.state != OctopipesState::Running {
                            break;
                        }
                        //TODO: read
                    }
                    //Exit
                }));
                //Set state to running
                self.state = OctopipesState::Running;
                Ok(())
            }
            OctopipesState::Running => Err(OctopipesError::ThreadAlreadyRunning),
            _ => Err(OctopipesError::NotSubscribed),
        }
    }

    /// ###  loop_stop
    ///
    /// `loop_stop` stops the client loop thread
    /// ```
    pub fn loop_stop(&mut self) -> Result<(), OctopipesError> {
        match self.state {
            OctopipesState::Running => {
                //Stop thread
                self.state = OctopipesState::Stopped;
                //Take joinable out of Option and then Join thread (NOTE: Using take prevents errors!)
                self.client_loop.take().map(thread::JoinHandle::join);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    //Subscribption functions

    /// ###  subscribe
    ///
    /// `subscribe` subscribe to Octopipes server; the client will subscribe to the groups described in the subscription_list

    pub fn subscribe(
        &mut self,
        subscription_list: &Vec<String>,
        mut cap_error: &OctopipesCapError,
    ) -> Result<(), OctopipesError> {
        //TODO: implement
        Err(OctopipesError::Unknown)
    }

    /// ###  unsubscribe
    ///
    /// `unsubscribe` unsubscribe from Octopipes server; if thread is running it will be stopped

    pub fn unsubscribe(&mut self) -> Result<(), OctopipesError> {
        //TODO: implement
        Err(OctopipesError::Unknown)
    }

    //Send message functions

    /// ###  send
    ///
    /// `send` sends a message to a certain remote

    pub fn send(&self, remote: &String, data: &Vec<u8>) -> Result<(), OctopipesError> {
        self.send_ex(remote, data, 0, OctopipesOptions::empty())
    }

    /// ###  send_ex
    ///
    /// `send_ex` sends a message to a certain remote with extended options

    pub fn send_ex(
        &self,
        remote: &String,
        data: &Vec<u8>,
        ttl: u8,
        options: OctopipesOptions,
    ) -> Result<(), OctopipesError> {
        //TODO: implement
        Err(OctopipesError::Unknown)
    }

    //Callbacks setters

    /// ###  set_on_received_callback
    ///
    /// `set_on_received_callback` sets the function to call on message received
    pub fn set_on_received_callback(
        &mut self,
        callback: fn(&OctopipesClient, Result<&OctopipesMessage, &OctopipesError>),
    ) {
        self.on_received_fn = Some(callback);
    }

    /// ###  set_on_sent_callbacl
    ///
    /// `set_on_sent_callbacl` sets the function to call when a message is sent
    pub fn set_on_sent_callback(&mut self, callback: fn(&OctopipesClient, &OctopipesMessage)) {
        self.on_sent_fn = Some(callback);
    }

    /// ###  set_on_subscribed
    ///
    /// `set_on_subscribed` sets the function to call on a successful subscription to the Octopipes Server
    pub fn set_on_subscribed(&mut self, callback: fn(&OctopipesClient)) {
        self.on_subscribed_fn = Some(callback);
    }

    /// ###  set_on_unsubscribed
    ///
    /// `set_on_unsubscribed` sets the function to call on a successful unsubscription from Octopipes server
    pub fn set_on_unsubscribed(&mut self, callback: fn(&OctopipesClient)) {
        self.on_unsubscribed_fn = Some(callback);
    }

}

impl Drop for OctopipesClient {
    fn drop(&mut self) {       
        //Stop thread
        self.loop_stop();
        drop(self);
    }
}
