# Source intent (verbatim)

[← index](README.md)

The user's own writing, preserved as the source of truth the synthesised chunks derive from.
Kept verbatim (including original phrasing/typos) so nothing is lost in paraphrase.

---

## I. Compositional Assembly (2026-09-10)

> Context: Compositional Assembly - Build higher order systems within a monadic container.
>
> A monad is a container that holds structures (terms) and processes (connectives).
> Decomposition and assembly seem like a gread dyad to add to the library. Is it me or does this seem to run parallel to sort and filter?
>
> Getting the foundations right is probably going to require some pretty advanced maths, as composition in its own right is an advanced subject, as is decomposition, assembly etc. - these needing to work combinitorically is quite advanced. For now we probably want to keep things simple. This should probably be simple joins, node to node and node to edge. Easy product as addition and subtraction I suppose.
>
> **Model:**
> A monad is represented in graph view as a k1 system. This is a container that contains structures (nodes) and processes (edges). Drawing an analagy with holochain for reference, the monad is the DNA, and the dyad of structures and processes are integrity and coordinator zomes.
>
> So we essentially treat a monad as a bucket of terms and connectives. These terms and connectives are loose units, but can be associated and joined into structures such as k2 and k3 graphs. Strictly speaking this should happen sequentially, so that first a dyad is formed and then dyads are connected into triads etc. as for example a tetrad must be composed of six dyads or four triads.
>
> However it may be that we start with a tetrad sketch, which should then be decomposed it into six dyads and four triads. Theres a good chance that the tetrad sketch may be wrong, and just be two seperate dyadic pairs missing proper coherence. It's therefore useful to be able to sketch the tetrad while also ensuring its decomposesed into subcomponests, possibly enabling us to delete the tetrad. More advanced operations, such as multiplying these two dyads to make a complimentary tetrad may be possible, but for now we should stick to simple assembly through an addition function (join).
>
> I'm not sure we can sequence graphs this way as we are not technically joining them or adding them to eachother. Technically this is a matter of serialising graph ordinality, dictated by order or type (rather than vertex ordinality).
>
> **Backend:**
> Composition (+) is an action of combining functions, Associativity (-) is a rule, identity (=) is the reconciler. (The layout can be corrected in the category theory triad). I would also like to plan this more deeply, as I'm not 100% certain on overything.
>
> **Backend ARCHITYPE validation:** We must assert the following as equivelent:
> Graph = System (k3=triad); Cardinality = coherence attribute (3,3 = dynamism, 4,6 = activity field); Order = Term designation (3=impulses, 4=sources); Size = Connective designation (3=acts, 4=interplays); Vertex Ordinality = Term position (1=term1, 2=term2); Edge Seriality = Connective position or sequence (1=connective1, 2=connective2). (these can be saved as hexads)
>
> **Backend TOPOLOGY INSTANCE validation:** Type = K4; Cardinality(n, n) = 4, 6; Order(n) = 4; Size(n) = 6; Ordinality(order) = Vertex(1, 2, 3, 4); Seriality(size) = Edge(1, 2, 3, 4, 5, 6)
>
> **Backend SYSTEM INSTANCE validation:** Coherence(Dynamism); TermDesignation(Impulses); ConnectiveDesignation(Acts); TermCharacters = <option3>; ConnectiveCharacters = <option3>
>
> **Backend composition:** Compose is not a single action, its meta, so we better just create smaller functions like addition and multiplication, though should make it easy for grouping these later with a coordinator zome. So compose system shouldnt exist until we have the functions to create a system from operations. The first of these would be the functions necessary to create the topology. I would assume we start by reading our validation rules and beginnig to compose the topology beginning with the assignment of vertex ordinality and edge seriality. We would then probably combine these through an addition function (?) for example vertex + edge + vertex = k2.
>
> It would seem that a k2, requires very simple rules of associativity: v1 and v2 are ordinals associated through edge1 which can be serialised before, between or after node articulation. We would not be able to associate v1 and v2 through edge5 for example.
>
> It would seem best to follow a serialisation process of assigning an vertex, then assigning and associating an edge, then assignig and associating an vertex. This is because topologies of a higher cardinality result in multiple edges, for example: K3 = vertex1 + edge1 + vertex2 + edge2 + vertex3 + edge 3= k3. While we have no associativity rules at the graph level, edge 1 must connect v1 and v2, which I believe is associativity - correct me if wrong. This associative ordering, especially with regards to the triad, also define the six laws algo:
> vertex1 + edge1 + vertex2 + edge2 + vertex3 + edge 3 = a 123 triad of order.
> vertex1 + edge3 + vertex3 + edge2 + vertex2 + edge 1 = a 132 triad of interaction. etc.
>
> This gets us as far as topological composition, but not as far as system composition. We return to our archetypal rules and ensure that the topological instance matches the system instance. We then compose a system in the same way as composing a topology; the assignment of terms and connectives, again probably assign and associate thorough addition. The basic form of triad would be composed as term 1 + connective 1 + term 2+ connective 2 + term 3 + connective 3 = triad. This would be incomplete however without anchoring terms to topology so that t1 to vertex1, t2 to vertex2 t3 to vertex3, connective1 to edge1, connective2 to edge2, connective3 to edge3. For a triad we would assert the coherence attribute is dynamism, term designation as impulses, connective designation as acts.
>
> As mentioned the coherence attribute should corresponde with a cardinality, term designations with order and connective designations to size. Term and connective designations should realate to coherence attribute, while order and size should relate to cardinality.
>
> **FRONTEND:** Nullad is a list of monad heads, just a set of containers that are sequence heads as entry points. You enter a monad to see its associated components (lets not call these associations). Monad workspace is a list showing the monads associated components and subsystems. This is a little tricky now i think of it as a term is easy to represent as a monad, while a connective would require two terms to enable an edge. But yes a multi select 'extract' and a compose/transform action (addition) would be needed. Composed systems just get processed into the next page from monad > dyad or from monad > triad (decomposed into dyads).
>
> **Phasing:** generally seems fine but worth reviewing again with the additional commentary.
>
> **LATER:** Cartesian product and other graph products. These will be important but need much consideration.
>
> **SOME SYSTEMS TO SEED:** Data / Material Science tetrad: Performance, Processing, Properties, Structure (Engineer between performance/processing; Scientist between structure/properties). Information science tetrad: Efficacy, attributes, representations, methods/workflows (characterised by evaluations). FAIR data principles tetrad (Findable, Accessible, Interoperable, Reusable). The interface state model is missing a triad, possibly interpretation(=), representation(-), standardisation(+). Convening triad: Hosting, Facilitating, Coordinating. Proof of Stake: Delegators(+), Validators(-), Stakeholders(=).

---

## II. Principled approach — functional construction (2026-09-11)

> Creating a new triad with nodes one, two, three and edges, I decompose it. When I hit join it recomposes into a triad but the nodes are incorrectly labeled. For now this is a proof of concept, this is basically the desired functionality. I dont think we want to do it this way though from a design perspective. It's more likely that they would be better off as a drag and drop interface, or something else. We have yet to figure that out. We can call this conjunction and disjunction most likely.
>
> I would actually treat where we are currently at as prototyping, with the plan basically being whats necessary to rebuild from scratch on more sturdy foundations. In other words we will scrap most of what we have built, as the code is probably a bomb site, treating it as an indicator of intent while recreating most of the functionality with cleaner architecture. So the plan is to get as precise intention down as possible. This will probably create a huge doc that will be difficult to review, so it may be more effective to assemble it from smaller chunks, which would basically be nodes and relations, as this system is itself a way of compressing intent at different degrees of resolution. A monolithic document is nearly impossible to read, its basically a monad that needs to be sorted into dyads and higher order systems.
>
> On the principaled approach, we are creating functionally. So we would want a function to create a vertex, and then a function to take two vertexes and create an edge between them. We would then want to be able to compose a single verted from the first function, with a pair of vertexes with an edge between them through what is assumedly addition, and then we would need to add two edges from this node to the other two nodes (this gives us a k3). We would then need to perform similar for a k4, by taking a k3 and adding a node, then linking to all other nodes. SO it would seem that to create complete graphs we need two functions... Create nodes, link created node to all other nodes in a set (is this valid it categrory theory? do sets exist?). We can generate the nodes sequentially, and we can also link them sequentially, in that 2 links to 1 first, then 3 links to 1 and then 2, which allows us to create a new system through addition where k3 + k1 = k4. This seems like a simple generative principal that can be used to complete all graphs sequentially.
>
> Yes starting from a node, we add a node, then we add a connection. This turns a k1 into a k2, from a k2 we can add a node and add a second connection, to get a path akin to one of the six laws (following 123 is technically expansion). The choice here is to complete a circuit (k3) by adding another connection, or continuing the path by adding a node and then another connection. Again we are faced with the choice of completing the tetrad by adding 3 more connections or continuing the path by adding another node and edge, offering the choice of completing the pentad by adding 6 more connectives. Again we can continue the path by adding another node and edge, which gives us the option of completing the hexad by adding 10 more connectives.
>
> A more elegant way of doing things is perhaps the approach of add node = k1, add node + connective = k2, add node and 2 connectives = k3, add a node and 3 connectives = k4, add a node and 4 connectives = k5, add a node and 5 connectives = k6, add a node and 6 connectives = k7. This is apparently the 'handshaking lemma' of n(n-1)/2.
>
> This lemma gives the odd result that we can construct graphs sequentially, so that in a k5 we have one k2 edge, two k3 edges, 3 k4 edges, and 4 k5 edges to make up the ten connectives. Each of these edges can be coloured, offering a number of interesting construction options to k6 at least. We K2 and K3 can remain consistent across our design space, K4>k5 has transition issues where we have to reconfigure the three edges from k3> k4 edges. From here we can continue the pattern up to k6. Maybe its better to describe as k2 connective is red, k3 connectives are one red two blue. k4 connectives are one red, two blue, three green. K5 connectives require reconfiguring the green so that one red and three greens form a box, and then orange connects the rest. K6 retains the K5 shape but adds a node and connects six new edges.
>
> If we complete the circuit with a third connective we get a walk (k3). Doing so would allow us to label the edges sequentially. We can add another node to the k3 using the first function, at which point to complete the graph we must add three more connections. Things get harder if we take two k2s, so that we generate the first k2 and then second k2.
>
> 123 for both nodes and edges:
> Sponsors, participants, stakeholders as nodes, hosting, coordinating, facilitating as edges.
> Startups, corporates, investors as nodes, supporting, finding, investing as edges.
</content>
