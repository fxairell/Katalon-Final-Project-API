<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>PATCH Method to edit a booking with invalid credentials</description>
   <name>PATCH A Booking (Invalid Credentials)</name>
   <tag></tag>
   <elementGuidId>6ba96c22-5669-4220-9b16-59987f0bc1c1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;John\&quot;,\n    \&quot;lastname\&quot; : \&quot;Doe\&quot;,\n    \&quot;totalprice\&quot; : 15000,\n    \&quot;depositpaid\&quot; : true,\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;2022-01-01\&quot;,\n        \&quot;checkout\&quot; : \&quot;2023-01-01\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;All Services\&quot;\n}&quot;,
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
      <value>Basic am9obmRvZTpUaGlzSXNOb3RBUGFzc3dvcmQ=</value>
      <webElementGuid>1f156ed0-3383-42c0-b187-b334dff6566e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
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
      <id>ca140ece-5356-40c0-9a17-c06301397f2d</id>
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
