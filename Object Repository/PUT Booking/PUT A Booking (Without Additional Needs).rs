<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>PUT Method to update a booking with valid credential without Additional Needs</description>
   <name>PUT A Booking (Without Additional Needs)</name>
   <tag></tag>
   <elementGuidId>3a335a33-0f18-40d5-9800-ffba533a0006</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;Jane\&quot;,\n    \&quot;lastname\&quot; : \&quot;Donut\&quot;,\n    \&quot;totalprice\&quot; : 45000,\n    \&quot;depositpaid\&quot; : false,\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;2024-01-01\&quot;,\n        \&quot;checkout\&quot; : \&quot;2025-01-01\&quot;\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bef9cd6f-b746-4bb5-8469-871b7342456a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>94cb33b5-72fa-455f-8a51-799a1a8e3d8c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic YWRtaW46cGFzc3dvcmQxMjM=</value>
      <webElementGuid>9696130e-2d13-4b50-b79b-579a5a84cd03</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
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
      <id>f4376e85-90ed-48c7-9ffb-cccacb07d752</id>
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

WS.verifyElementPropertyValue(response, 'firstname', 'Jane')
WS.verifyElementPropertyValue(response, 'lastname', 'Donut')
WS.verifyElementPropertyValue(response, 'totalprice', '45000')
WS.verifyElementPropertyValue(response, 'depositpaid', 'false')
WS.verifyElementPropertyValue(response, 'bookingdates.checkin', '2024-01-01')
WS.verifyElementPropertyValue(response, 'bookingdates.checkout', '2025-01-01')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
