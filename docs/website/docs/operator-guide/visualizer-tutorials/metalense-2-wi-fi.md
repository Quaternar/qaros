---
title: Put METALENSE 2 on Wi-Fi
sidebar_position: 4
description: Join METALENSE 2 glasses to a Wi-Fi network over USB from the Visualizer, so they can reach the Hub and join it.
---

# Put METALENSE 2 on Wi-Fi

Typing a Wi-Fi password on the glasses is slow. The Visualizer sets the network over the USB cable
instead. Once on the network, the glasses join the session by themselves after you pair them.

## Before you start

- The glasses are connected over a USB-C data cable and the player is installed
  ([Install and Pair](./quest-and-metalense-2.md)).
- **Session** › **Devices** › **Set up or update a device** › **Device type** = **Metalens · USB**,
  and the glasses are listed under **Connected over USB**.
- The network is WPA2 (or open). Enterprise (802.1X) networks are not supported.

## Change the network

1. Open the step **Wi-Fi on the glasses** (step 3). It shows *Checking Wi-Fi on the glasses*, then
   the form. Without glasses on the cable it asks for them instead, as here:

   ![Wi-Fi on the glasses step, before the glasses are plugged in](../../img/visualizer/metalens-wifi.png)

2. If it says *Wi-Fi is off on the glasses*, click **Turn on** (or just continue: **Join** turns it
   on).
3. Type the network name in **Network**, or open **Saved** and pick one. Networks this PC knows are
   offered, the one the PC is on first.
4. Set **Security** (**WPA2** or **Open**) and type the **Password**. It is typed every time: the
   Visualizer never stores or logs it and does not read saved keys from Windows.
5. Click **Join**. It shows *Joining &lt;network&gt;*.
6. When the glasses report the network, the card shows **On** *&lt;network&gt;* · *&lt;address&gt;*
   in green, with **Copy**.
7. Pair the glasses in the next step, **Pair the glasses**
   ([Pair a Headset That Joins by Itself](./pair-a-headset.md)).

To move the glasses to another network later, click **Disconnect** (the glasses forget the
current network) and repeat from step 3. **Turn Wi-Fi off** switches Wi-Fi off on the glasses.

## If it does not work

| The card says | Meaning |
|---|---|
| *Connect the glasses over USB to set their Wi-Fi.* | The cable is unplugged or charging-only |
| *Set Wi-Fi on the glasses: they do not accept a network over USB.* | The glasses' Android is older than 11. Set Wi-Fi on the glasses, then **Check again** |
| A notification after **Join**, password field empty again | Wrong password. Type it again |
