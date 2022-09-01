# Barcodes-php

尝试用rust给php写扩展，主要是用来学习的。

## Necessary

- **rust** 1.56 or later
- **libclang** 9.0 or later
- **php** 7.0 or later

## Usage

```php
<?php

use BarCodes\Code39;
use BarCodes\Codabar;
use BarCodes\Code128;

$code = new Code39();
$code->setForeground([255, 0, 0, 255]);
$code->generatePng("1ISTHELONELIESTNUMBER", "my_barcode.png");


$code = new Codabar();
$code->generateJpeg("1A43BD4D", "my_barcode.jpg");

$code = new Code128();
$code->setBackground([0, 0, 255, 255]);
$code->generateGif("ƁxyZÀ199!*1", "my_barcode.gif");

```

## Screenshots

![image](https://github.com/chenhuaiyuan/barcodes-php/raw/master/example/my_barcode.png)
![image](https://github.com/chenhuaiyuan/barcodes-php/raw/master/example/my_barcode.jpg)
![image](https://github.com/chenhuaiyuan/barcodes-php/raw/master/example/my_barcode.gif)
