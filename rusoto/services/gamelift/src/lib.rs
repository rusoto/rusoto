// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><fullname>Amazon GameLift Service</fullname> <p> Amazon GameLift provides a range of multiplayer game hosting solutions. As a fully managed service, GameLift helps you:</p> <ul> <li> <p>Set up EC2-based computing resources and use GameLift FleetIQ to and deploy your game servers on low-cost, reliable Spot instances.</p> </li> <li> <p>Track game server availability and route players into game sessions to minimize latency.</p> </li> <li> <p>Automatically scale your resources to meet player demand and manage costs</p> </li> <li> <p>Optionally add FlexMatch matchmaking.</p> </li> </ul> <p>With GameLift as a managed service, you have the option to deploy your custom game server or use Amazon GameLift Realtime Servers to quickly stand up lightweight game servers for your game. Realtime Servers provides an efficient game server framework with core Amazon GameLift infrastructure already built in.</p> <p> <b>Now in Public Preview:</b> </p> <p>Use GameLift FleetIQ as a standalone feature with EC2 instances and Auto Scaling groups. GameLift FleetIQ provides optimizations that make low-cost Spot instances viable for game hosting. This extension of GameLift FleetIQ gives you access to these optimizations while managing your EC2 instances and Auto Scaling groups within your own AWS account.</p> <p> <b>Get Amazon GameLift Tools and Resources</b> </p> <p>This reference guide describes the low-level service API for Amazon GameLift and provides links to language-specific SDK reference topics. See also <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-components.html"> Amazon GameLift Tools and Resources</a>.</p> <p> <b>API Summary</b> </p> <p>The Amazon GameLift service API includes two key sets of actions:</p> <ul> <li> <p>Manage game sessions and player access -- Integrate this functionality into game client services in order to create new game sessions, retrieve information on existing game sessions; reserve a player slot in a game session, request matchmaking, etc.</p> </li> <li> <p>Configure and manage game server resources -- Manage your Amazon GameLift hosting resources, including builds, scripts, fleets, queues, and aliases. Set up matchmakers, configure auto-scaling, retrieve game logs, and get hosting and game metrics.</p> </li> </ul> <p> <b> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-awssdk.html"> Task-based list of API actions</a> </b> </p></p>
//!
//! If you're using the service, you're probably looking for [GameLiftClient](struct.GameLiftClient.html) and [GameLift](trait.GameLift.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
