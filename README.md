# autodiscover

Allows email server autodiscovery with Outlook and Thunderbird on non-Exchange servers.

If you host an email server, you want it to be compatible with all kinds of clients. You don't want your end user to **enter manually the email configuration** for their email address.

With Thunderbird and software using the same standards, you can just use the [autoconfig](https://developer.mozilla.org/en-US/docs/Mozilla/Thunderbird/Autoconfiguration) file. Sadly, on Outlook, Apple Mail and Apple phones, it doesn't work the same.

Outlook uses an undocumented, proprietary protocol called **Autodiscover**, that is made to only work on Outlook Exchange servers. This software attempts to reimplement this protocol, so your free-software based mail server works out-of-the-box with Outlook clients.

Apple mobile phones uses the mobileconfig file which is again, poorly documented.

## Supported clients
- Thunderbird
- Apple Mail
- (Not yet tested) Outlook clients

## Setup

### The application

Git clone then build with Cargo, like a regular Rust application.

### The server

Run the application on your server, then make it listen to those two virtual hosts:
- autodiscover.mydomain.com
- autoconfig.mydomain.com

The best way to do so is to use a reverse proxy, which can also handle HTTPS.

### DNS records

You have to create a lot of DNS records to make autodiscovery work for all clients. Be sure to double check every entry.

- Replace `mydomain.com` with your own domain.
- Replace `1.2.3.4` with your server's IP address.

#### On the mail server's domain

All of those entries should be recorded on your main domain.

```
autoconfig          86400 IN CNAME  autodiscover.mydomain.com.
autodiscover        86400 IN CNAME  autodiscover.mydomain.com
_imap._tcp          86400 IN SRV    0 0 143 mail.mydomain.com.
_imaps._tcp         86400 IN SRV    0 0 993 mail.mydomain.com.
_pop3._tcp          86400 IN SRV    0 0 110 mail.mydomain.com.
_pop3s._tcp         86400 IN SRV    0 0 995 mail.mydomain.com.
_smtp._tcp          86400 IN SRV    0 0 25 mail.mydomain.com.
_smtps._tcp         86400 IN SRV    0 0 465 mail.mydomain.com.
_submission._tcp    86400 IN SRV    0 0 587 mail.mydomain.com.
_autodiscover._tcp  86400 IN SRV 	5 0 443 autodiscover.mydomain.com
```

#### On a "customer" domain

Those entries should be added to any other domain using your mail server.

```
autoconfig          86400 IN CNAME 	autodiscover.mydomain.com
autodiscover        86400 IN CNAME 	autodiscover.mydomain.com
_autodiscover._tcp  86400 IN SRV 	5 0 443 autodiscover.mydomain.com
```

You also need to generate a TLS certificate for each customer domain (specifically, for autoconfig and autodiscover subdomains).


This project is fully and shamelessly inspired of the following projects:
- https://github.com/Earl0fPudding/outlook-autodiscover-nginx
- https://github.com/gronke/email-autodiscover
- https://github.com/Tiliq/autodiscover.xml

This software is a rewrite of those projects in Rust with the [Actix](https://actix.rs) framework, so no PHP interpreter or Javascript is needed.

