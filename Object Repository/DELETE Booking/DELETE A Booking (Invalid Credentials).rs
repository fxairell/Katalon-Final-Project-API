<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>DELETE Method to erase a booking with invalid credential</description>
   <name>DELETE A Booking (Invalid Credentials)</name>
   <tag></tag>
   <elementGuidId>0b85cb2b-fb71-4887-af04-8f1e442d1920</elementGuidId>
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
      <webElementGuid>5f486c09-ffe7-4cde-b1d9-fc1ac0d42138</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic am9obmRvZTpUaGlzSXNOb3RBUGFzc3dvcmQ=</value>
      <webElementGuid>619056e1-3fff-4c24-a6d2-96566d30b0f7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
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
      <defaultValue>GlobalVariable.booking_id</defaultValue>
      <description></description>
      <id>49aed23c-e09b-4fb9-958b-e39f83fd6548</id>
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

// Forbidden because account is not authorized
WS.verifyResponseStatusCode(response, 403)
assertThat(response.getStatusCode()).isEqualTo(403)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
