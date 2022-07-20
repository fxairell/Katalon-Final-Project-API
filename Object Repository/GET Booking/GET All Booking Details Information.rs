<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>GET Method to get all booking details information by Booking ID</description>
   <name>GET All Booking Details Information</name>
   <tag></tag>
   <elementGuidId>3a3ccbd5-5b69-433b-8206-23cd54920a02</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5fb90ebb-759d-43ef-be86-24f0c74b46be</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://restful-booker.herokuapp.com/booking/${bookingid}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>45</defaultValue>
      <description></description>
      <id>f57b940b-4322-4fc9-abc0-9cd9a84f4411</id>
      <masked>false</masked>
      <name>bookingid</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

assertThat(response.getResponseText()).contains('firstname')
assertThat(response.getResponseText()).contains('lastname')
assertThat(response.getResponseText()).contains('totalprice')
assertThat(response.getResponseText()).contains('depositpaid')
assertThat(response.getResponseText()).contains('checkin')
assertThat(response.getResponseText()).contains('checkout')
assertThat(response.getResponseText()).contains('additionalneeds')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
