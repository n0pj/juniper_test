-- MySQL dump 10.13  Distrib 8.0.16, for osx10.14 (x86_64)
--
-- Host: 127.0.0.1    Database: rust_graphql
-- ------------------------------------------------------
-- Server version	8.0.15
/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */
;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */
;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */
;
SET NAMES utf8mb4;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */
;
/*!40103 SET TIME_ZONE='+00:00' */
;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */
;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */
;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */
;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */
;
--
-- Table structure for table `user`
--
DROP TABLE IF EXISTS `user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */
;
SET character_set_client = utf8mb4;
CREATE TABLE `user` (
  `id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `test1` varchar(255),
  `test2` varchar(255),
  `test3` varchar(255),
  `test4` varchar(255),
  `test5` varchar(255),
  `test6` varchar(255),
  `test7` varchar(255),
  `test8` varchar(255),
  `test9` varchar(255),
  `test10` varchar(255),
  `test11` varchar(255),
  `test12` varchar(255),
  `test13` varchar(255),
  `test14` varchar(255),
  `test15` varchar(255),
  `test16` varchar(255),
  `test17` varchar(255),
  `test18` varchar(255),
  `test19` varchar(255),
  `test20` varchar(255),
  `test21` varchar(255),
  `test22` varchar(255),
  `test23` varchar(255),
  `test24` varchar(255),
  `test25` varchar(255),
  `test26` varchar(255),
  `test27` varchar(255),
  `test28` varchar(255),
  `test29` varchar(255),
  `test30` varchar(255),
  `test31` varchar(255),
  `test32` varchar(255),
  `test33` varchar(255),
  `test34` varchar(255),
  `test35` varchar(255),
  `test36` varchar(255),
  `test37` varchar(255),
  `test38` varchar(255),
  `test39` varchar(255),
  `test40` varchar(255),
  `test41` varchar(255),
  `test42` varchar(255),
  `test43` varchar(255),
  `test44` varchar(255),
  `test45` varchar(255),
  `test46` varchar(255),
  `test47` varchar(255),
  `test48` varchar(255),
  `test49` varchar(255),
  `test50` varchar(255),
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */
;
--
-- Dumping data for table `user`
--
LOCK TABLES `user` WRITE;
/*!40000 ALTER TABLE `user` DISABLE KEYS */
;
/*!40000 ALTER TABLE `user` ENABLE KEYS */
;
UNLOCK TABLES;
--
-- Table structure for table `product`
--
DROP TABLE IF EXISTS `product`;
/*!40101 SET @saved_cs_client     = @@character_set_client */
;
SET character_set_client = utf8mb4;
CREATE TABLE `product` (
  `id` varchar(255) NOT NULL,
  `user_id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `price` decimal(10, 0) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `product_fk0` (`user_id`),
  CONSTRAINT `product_fk0` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */
;
--
-- Dumping data for table `product`
--
LOCK TABLES `product` WRITE;
/*!40000 ALTER TABLE `product` DISABLE KEYS */
;
/*!40000 ALTER TABLE `product` ENABLE KEYS */
;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */
;
/*!40101 SET SQL_MODE=@OLD_SQL_MODE */
;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */
;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */
;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */
;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */
;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */
;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */
;