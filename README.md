# autodiscover

Allows email server autodiscovery with Outlook and Thunderbird on non-Exchange servers.

If you host an email server, you want it to be compatible with all kinds of clients. You don't want your end user to **enter manually the email configuration** for their email address.

With Thunderbird and software using the same standards, you can just use the [autoconfig](https://developer.mozilla.org/en-US/docs/Mozilla/Thunderbird/Autoconfiguration) file. Sadly, on Outlook, Apple Mail and Apple phones, it doesn't work the same.

Outlook uses an undocumented, proprietary protocol called **Autodiscover**, that is made to only work on Outlook Exchange servers. This software attempts to reimplement this protocol, so your free-software based mail server works out-of-the-box with Outlook clients.

Apple mobile phones uses the mobileconfig file which is again, poorly documented.

## Supported clients
- (Not yet tested) Outlook clients, Thunderbird and free software alternatives (KDE mail clients), Apple Mail, Apple phone mail app

## Setup

### The application

Git clone then build with Cargo, like a regular Rust application.

### The server

Run the application on a virtual host on your server, like `autodiscover.example.com`. 

### DNS records

You have to create a lot of DNS records to make autodiscovery work for all clients. Be sure to double check every entry.

#### On the mail server's domain

All of those entries should be recorded on your main domain.

(To complete)

#### On a "customer" domain

Those entries should be added to any other domain using your mail server.

(To complete)



This project is fully and shamelessly inspired of the following projects:
- https://github.com/Earl0fPudding/outlook-autodiscover-nginx
- https://github.com/gronke/email-autodiscover
- https://github.com/Tiliq/autodiscover.xml

This software is a rewrite of those projects in Rust with the [Actix](https://actix.rs) framework, so no PHP interpreter or Javascript is needed.

