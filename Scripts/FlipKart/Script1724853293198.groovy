import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.flipkart.com/')

WebUI.verifyElementPresent(findTestObject('FlipKartObject/Page_Online Shopping Site for Mobiles, Elec_b3f752/Page_Online Shopping Site for Mobiles, Electronics, Furniture, Grocery, Lifestyle, Books  More. Best Offers/img_Grocery__2puWtW _3a3qyb'), 
    0)

WebUI.click(findTestObject('FlipKartObject/Page_Online Shopping Site for Mobiles, Elec_b3f752/Page_Online Shopping Site for Mobiles, Electronics, Furniture, Grocery, Lifestyle, Books  More. Best Offers/img_Grocery__2puWtW _3a3qyb'))

WebUI.verifyElementPresent(findTestObject('FlipKartObject/Page_Online Shopping Site for Mobiles, Elec_b3f752/Page_Online Shopping Site for Mobiles, Electronics, Furniture, Grocery, Lifestyle, Books  More. Best Offers/Page_Mobile Phones Online at Best Prices in India/span_Electronics'), 
    10)

WebUI.click(findTestObject('FlipKartObject/Page_Online Shopping Site for Mobiles, Elec_b3f752/Page_Online Shopping Site for Mobiles, Electronics, Furniture, Grocery, Lifestyle, Books  More. Best Offers/Page_Mobile Phones Online at Best Prices in India/span_Electronics'))

WebUI.click(findTestObject('Object Repository/Page_Mobile Phones Online at Best Prices in India/span_Men'))

WebUI.click(findTestObject('Object Repository/Page_Mobile Phones Online at Best Prices in India/a_Sports Shoes'))

WebUI.click(findTestObject('Object Repository/Page_Sports Shoes For Men - Upto 50 to 80 O_648e13/div_Brand'))

WebUI.click(findTestObject('Object Repository/Page_Sports Shoes For Men - Upto 50 to 80 O_648e13/div_Brand_XqNaEv'))

WebUI.click(findTestObject('Object Repository/Page_Sports Shoes For Men - Upto 50 to 80 O_648e13/div_30 or more'))

WebUI.click(findTestObject('Object Repository/Page_Sports Shoes For Men - Upto 50 to 80 O_648e13/div_Brand_XqNaEv'))

WebUI.click(findTestObject('Object Repository/Page_Sports Shoes For Men - Upto 50 to 80 O_648e13/span_Women'))

WebUI.click(findTestObject('Object Repository/Page_Sports Shoes For Men - Upto 50 to 80 O_648e13/a_Topwear'))

WebUI.click(findTestObject('Object Repository/Page_Topwear - Buy Topwear For Men, Women  _d34f92/div_Gender_XqNaEv eJE9fb'))

WebUI.click(findTestObject('Object Repository/Page_Topwear - Buy Topwear For Men, Women  _d34f92/span_Clear all'))

