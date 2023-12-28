CREATE TABLE blog (
    id INTEGER PRIMARY KEY,
    title TEXT,
    summary TEXT,
    body TEXT,
    date DATE,
    tags TEXT[]
);

INSERT INTO blog (id, title, summary, body, date, tags)
VALUES (
    1,
    'Monitor your houseplants with a NodeMCU ESP8266 microcontroller and a capacitive sensor 🌱',
    'Brief summary here',
    '
    Often, it''s hard to observe moisture in soil, making it hard to judge whether your houseplants need water. This is a problem that can be countered with sensors. Of course, sensors need to be hooked up to a device to interpret and process signals. I was looking for a lightweight Arduino-like piece of hardware with integrated Wi-Fi and found the ESP8266. The fact that the board is open-source, programmable through the Arduino IDE and costs under €5 make it an appealing choice for a first internet-of-things project. Its specifications can be found [here](https://components101.com/development-boards/nodemcu-esp8266-pinout-features-and-datasheet#:~:text=NodeMCU%20is%20an%20open%2Dsource,on%20the%20ESP%2D12%20module.) and to give you an idea what else you can make check [this](https://randomnerdtutorials.com/projects-esp8266/) out.

    &nbsp;

    Soil moisture is the amount of water that is present in the soil. A capacitive soil moisture sensor detects and measures anything that influences its capacitance, such as water. There are also alternatives, such as a conductivity sensor which has two electrodes that get voltage applied. A change in conductivity can be attributed to a change in the amount of water present in soil, as water increases conductivity. Then there are also [leaf sensors](https://en.wikipedia.org/wiki/Leaf_sensor#:~:text=A%20leaf%20sensor%20is%20a,moisture%20level%20in%20plant%20leaves.) that measure hydrostatic pressure in plant cells, which increases when the plant is hydrated and vice versa. Eventually I went with the capacitive sensor because they''re cheap, have a faster response and have lower noise compared to the alternatives.

    &nbsp;

    ![ESP8266 pinout](../images/esp8266-pinout.jpeg)

    &nbsp;

    For this project, we actually only need the analog pin to read from the sensor, and the `3.3v` and `gnd` (ground) pin to supply power. The board can be powered through its micro-USB port, but it''s also possible to hook it up to a [lithium polymer (LiPo) battery](https://www.aliexpress.com/w/wholesale-lipo-battery.html), which are also commonly found in phones. I used a [charging module](https://www.aliexpress.com/w/wholesale-arduino-battery-charger-module.html) to recharge the battery from time to time. The following schematic shows how these parts come together.

    &nbsp;

    ![Schematic](../images/soilmoisturemeter-diagram.png)

    &nbsp;

    If you want to get started with the ESP8266 or a similar board, I highly recommend to consult websites such as [instructables](https://www.instructables.com) or [random nerd tutorials](https://randomnerdtutorials.com/) for tutorials and inspiration.
    ',
    '2022-08-01',
    ARRAY['tag1','tag2']
);
