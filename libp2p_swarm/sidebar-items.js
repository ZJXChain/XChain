initSidebarItems({"enum":[["AddAddressResult","The result of adding an address to an ordered list of addresses with associated scores."],["AddressScore","The “score” of an address w.r.t. an ordered collection of addresses."],["CloseConnection","The options which connections to close."],["DialError","The possible failures of dialing."],["DialPeerCondition","The available conditions under which a new dialing attempt to a peer is initiated when requested by [`NetworkBehaviourAction::DialPeer`]."],["NetworkBehaviourAction","An action that a [`NetworkBehaviour`] can trigger in the `Swarm` in whose context it is executing."],["NotifyHandler","The options w.r.t. which connection handler to notify of an event."],["SwarmEvent","Event generated by the `Swarm`."]],"mod":[["protocols_handler","Once a connection to a remote peer is established, a `ProtocolsHandler` negotiates and handles one or more specific protocols on the connection."],["toggle",""]],"struct":[["AddressRecord","An record in a prioritised list of addresses."],["DummyBehaviour","Dummy implementation of [`NetworkBehaviour`] that doesn’t do anything."],["IntoProtocolsHandlerSelect","Implementation of `IntoProtocolsHandler` that combines two protocols into one."],["OneShotHandler","A `ProtocolsHandler` that opens a new substream for each request."],["OneShotHandlerConfig","Configuration parameters for the `OneShotHandler`"],["ProtocolsHandlerSelect","Implementation of `ProtocolsHandler` that combines two protocols into one."],["Swarm","Contains the state of the network, plus the way it should behave."],["SwarmBuilder","A `SwarmBuilder` provides an API for configuring and constructing a `Swarm`, including the underlying [`Network`]."],["SwarmPollParameters","Parameters passed to `poll()`, that the `NetworkBehaviour` has access to."]],"trait":[["NetworkBehaviour","A behaviour for the network. Allows customizing the swarm."],["NetworkBehaviourEventProcess","When deriving [`NetworkBehaviour`] this trait must by default be implemented for all the possible event types generated by the inner behaviours."],["PollParameters","Parameters passed to `poll()`, that the `NetworkBehaviour` has access to."]],"type":[["NegotiatedSubstream","Substream for which a protocol has been chosen."]]});